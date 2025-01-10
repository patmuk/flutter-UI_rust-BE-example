use log::trace;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::application::api::lifecycle::{
    AppConfig, AppState, AppStatePersistError, AppStatePersister, ProcessingError,
};

// use super::app_state_persistance_error::AppStatePersistError;

pub(crate) struct UnititializedAppStateFilePersister {}

#[derive(Debug)]
pub(crate) struct AppStateFilePersister {
    pub(crate) path: PathBuf,
}

// handle errors as suggested by https://kazlauskas.me/entries/errors
#[derive(thiserror::Error, Debug)]
pub(crate) enum AppStateFilePersisterError {
    #[error("Cannot read the file from path: {1}")]
    IOError(#[source] io::Error, PathBuf),
    #[error("Cannot create directory to store persistance file in {1}")]
    IODirError(#[source] io::Error, PathBuf),
    #[error("could not understand (=deserialize) the file {1}. Maybe it's content got corrupted? Bincode-Error: {0}")]
    DeserializationError(#[source] bincode::Error, PathBuf),
    #[error("could not write (=serialize) the file {1}. Maybe it's content got corrupted? Bincode-Error: {0}")]
    SerializationError(#[source] bincode::Error, PathBuf),
    #[error("No File found in: {0}")]
    FileNotFound(PathBuf),
}

impl AppStatePersistError for AppStateFilePersisterError {
    fn from_io_error(err: std::io::Error, path: PathBuf) -> Self {
        if err.kind() == io::ErrorKind::NotFound {
            AppStateFilePersisterError::FileNotFound(path)
        } else {
            AppStateFilePersisterError::IOError(err, path)
        }
    }

    fn from_deserialization_error(err: bincode::Error, path: PathBuf) -> Self {
        AppStateFilePersisterError::DeserializationError(err, path)
    }

    fn from_serialization_error(err: bincode::Error, path: PathBuf) -> Self {
        AppStateFilePersisterError::SerializationError(err, path)
    }

    fn to_processing_error(&self) -> crate::application::api::lifecycle::ProcessingError {
        match self {
            AppStateFilePersisterError::IOError(_err, path)
            | AppStateFilePersisterError::IODirError(_err, path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_string_lossy().to_string(),
            },
            AppStateFilePersisterError::DeserializationError(_err, path)
            | AppStateFilePersisterError::SerializationError(_err, path) => {
                ProcessingError::NotPersisted {
                    error: self.to_string(),
                    url: path.to_string_lossy().to_string(),
                }
            }
            AppStateFilePersisterError::FileNotFound(path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_string_lossy().to_string(),
            },
        }
    }
}

/// Persists the application state to storage (a file).
/// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
impl AppStatePersister for AppStateFilePersister {
    fn new<AC: AppConfig, ASPE: AppStatePersistError>(app_config: &AC) -> Result<Self, ASPE> {
        // fn new(app_config: &AppConfigImpl) -> Result<Self, AppStatePersistError> {
        // create the directories, but no need to write the file, as there is only the default content
        // remove the last part, as this is the file
        let path = PathBuf::from(app_config.get_app_state_url());
        let directories = path
            .components()
            .take(path.components().count() - 1)
            .collect::<PathBuf>();
        create_dir_all(directories)
            // .map_err(|io_err| PersisterError::IODirError(io_err, path.clone()))?;
            .map_err(|io_err| ASPE::from_io_error(io_err, path.clone()))?;
        Ok(AppStateFilePersister {
            path: path.to_owned(),
        })
    }

    fn persist_app_state<AS: AppState + Serialize, ASPE: AppStatePersistError>(
        &self,
        app_state: &AS,
    ) -> Result<(), ASPE> {
        // trace!(
        //     "persisting app state:\n  {app_state:?}\n to {:?}",
        //     self.path
        // );
        let serialized_app_state: Vec<u8> = bincode::serialize(app_state).expect(
            "bincode.serialzation itself should not result in an error, \
    unless the contract with serde is not respected!",
        );
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent).map_err(|error| {
                // AppStateFilePersisterError::IOError(error, self.path.to_owned())
                ASPE::from_io_error(error, self.path.to_owned())
            })?;
        }
        File::create(&self.path)
            .and_then(|mut file| file.write_all(&serialized_app_state))
            // .map_err(|ioerr| AppStateFilePersisterError::IOError(ioerr, self.path.to_owned()))?;
            .map_err(|ioerr| ASPE::from_io_error(ioerr, self.path.to_owned()))?;
        trace!("Persisted app state to file: {:?}", self.path);
        Ok(())
    }

    // get the last persisted app state from a file, if any exists, otherwise creates a new app state
    // this function is only called once, in the initialization/app state constructor
    fn load_app_state<
        AC: AppConfig,
        AS: AppState + for<'a> Deserialize<'a>,
        ASPE: AppStatePersistError,
    >(
        &self,
    ) -> Result<AS, ASPE> {
        trace!("loading the app state from {:?}", self.path);
        let loaded = std::fs::read(&self.path)
            .map_err(|error| ASPE::from_io_error(error, self.path.to_owned()))?;
        let app_state = bincode::deserialize(&loaded).map_err(|e| {
            ASPE::from_deserialization_error(e, self.path.to_path_buf())
            // AppStateFilePersisterError::DeserializationError(e, self.path.to_path_buf())
        })?;
        Ok(app_state)
    }
}

#[cfg(test)]
mod tests {
    // don't execute the tests in parallel, as file access would lead to race conditions
    use serial_test::serial;
    use std::fs::{create_dir_all, File};
    use std::io::ErrorKind as IoErrorKind;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use crate::application::api::lifecycle::Cqrs;
    use crate::application::api::lifecycle::{
        AppConfig, AppState, AppStatePersister, TodoListModelCommand,
    };
    use crate::application::app_config::AppConfigImpl;
    use crate::application::app_state::AppStateImpl;
    use crate::infrastructure::app_state_file_persister::AppStateFilePersisterError;
    // use crate::infrastructure::app_state_persistance_error::AppStatePersistError;

    use super::AppStateFilePersister;

    /// Path to the temporary test directory
    static TEST_FILE: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from("delme/temptest/test_app_state_bin"));

    /// Clean up the test directory after running tests
    fn cleanup_test_file() {
        if TEST_FILE.exists() {
            let root_dir = TEST_FILE
                .components()
                .next()
                .expect("path includes directory names");
            std::fs::remove_dir_all(root_dir).expect("Could not delete test file");
        }
    }
    fn create_test_app_config() -> AppConfigImpl {
        AppConfig::new(Some(TEST_FILE.to_string_lossy().to_string()))
    }
    fn create_test_app_state() -> AppStateImpl {
        let app_state = AppStateImpl::new(&create_test_app_config());
        let command = TodoListModelCommand::AddTodo("Test TODO".to_string());
        command.process();
        app_state
    }
    fn create_test_persister() -> AppStateFilePersister {
        // AppStateFilePersister::new(&create_test_app_config())
        AppStateFilePersister::new::<AppConfigImpl, AppStateFilePersisterError>(
            &create_test_app_config(),
        )
        .expect("Persister should have been created.")
    }
    fn assert_eq_app_states(left: &AppStateImpl, right: &AppStateImpl) {
        assert_eq!(
            left.todo_category_model_lock
                .lock
                .blocking_read()
                .get_title(),
            right
                .todo_category_model_lock
                .lock
                .blocking_read()
                .get_title()
        );
        assert_eq!(
            left.todo_list_model_lock
                .lock
                .blocking_read()
                .get_todos_as_string(),
            right
                .todo_list_model_lock
                .lock
                .blocking_read()
                .get_todos_as_string()
        );
    }

    #[test]
    #[serial]
    fn read_existing_file() {
        let original = create_test_app_state();
        let persister = create_test_persister();
        persister.persist_app_state::<AppStateImpl, AppStateFilePersisterError>(&original);
        let loaded = persister
            .load_app_state::<AppConfigImpl, AppStateImpl, AppStateFilePersisterError>()
            .expect("App state not loaded");

        assert_eq_app_states(&original, &loaded);
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn overwrite_existing_file() -> std::result::Result<(), std::io::Error> {
        let original = create_test_app_state();
        let persister = create_test_persister();
        persister
            .persist_app_state::<AppStateImpl, AppStateFilePersisterError>(&original)
            .expect("App state not persisted");
        assert!(&TEST_FILE.exists());

        let changed_app_state = create_test_app_state();
        let change_command = TodoListModelCommand::AddTodo("added todo".to_string());
        change_command.process();
        persister.persist_app_state::<AppStateImpl, AppStateFilePersisterError>(&changed_app_state);

        let loaded = persister
            .load_app_state::<AppConfigImpl, AppStateImpl, AppStateFilePersisterError>()
            .expect("AppState should have been loaded.");
        assert_eq_app_states(&changed_app_state, &loaded);
        cleanup_test_file();
        Ok(())
    }

    #[test]
    #[serial]
    fn overwrite_corrupted_file() -> std::result::Result<(), std::io::Error> {
        if let Some(parent) = TEST_FILE.parent() {
            create_dir_all(parent).unwrap();
        }
        File::create(&*TEST_FILE)
            .unwrap()
            .write_all(b"corrupted")
            .unwrap();

        let persister = create_test_persister();
        let changed_app_state = create_test_app_state();
        let change_command = TodoListModelCommand::AddTodo("added todo".to_string());
        change_command.process();
        persister.persist_app_state::<AppStateImpl, AppStateFilePersisterError>(&changed_app_state);

        let loaded = persister
            .load_app_state::<AppConfigImpl, AppStateImpl, AppStateFilePersisterError>()
            .expect("AppState should have been loaded.");

        assert_eq_app_states(&changed_app_state, &loaded);
        cleanup_test_file();
        Ok(())
    }

    #[test]
    #[serial]
    fn load_corrupted_data() {
        if let Some(parent) = TEST_FILE.parent() {
            create_dir_all(parent).unwrap();
        }
        std::fs::write(&*TEST_FILE, "corrupted").unwrap();
        let persister = create_test_persister();
        let result =
            persister.load_app_state::<AppConfigImpl, AppStateImpl, AppStateFilePersisterError>();
        assert!(&result.is_err());
        use bincode::ErrorKind;
        assert!(matches!(
            result,
            Err(AppStateFilePersisterError::DeserializationError(ref err, _))
                if {
                   match &**err {
                       ErrorKind::Io(ref inner_io_err) => {
                           inner_io_err.kind() == IoErrorKind::UnexpectedEof
                       },
                       _ => false,
                   }
                }
        ));
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn write_new_file() {
        cleanup_test_file();
        assert!(!TEST_FILE.exists());
        let _ = create_test_app_state();

        assert!(TEST_FILE.exists());
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn test_load_not_existing_file() {
        cleanup_test_file();
        assert!(!TEST_FILE.exists());
        let persister = create_test_persister();
        let result =
            persister.load_app_state::<AppConfigImpl, AppStateImpl, AppStateFilePersisterError>();

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AppStateFilePersisterError::FileNotFound(_))
        ));
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn cleanup() {
        cleanup_test_file();
    }
}
