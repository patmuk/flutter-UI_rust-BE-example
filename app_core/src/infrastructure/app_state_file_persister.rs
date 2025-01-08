use log::trace;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::application::api::lifecycle::{
    AppConfig, AppStatePersister, UnititializedAppStatePersister,
};
use crate::application::app_config::AppConfigImpl;
use crate::application::app_state::AppStateImpl;

use super::app_state_persistance_error::AppStatePersistError;

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
    #[error("could not understand (=deserialize) the file {1}. Maybe it's content got corrupted?")]
    DeserializationError(#[source] bincode::Error, PathBuf),
    #[error("No File found in: {0}")]
    FileNotFound(PathBuf),
}

impl UnititializedAppStatePersister for UnititializedAppStateFilePersister {
    type AppConfig = AppConfigImpl;
    type AppStatePersisterImplementation = AppStateFilePersister;
    // fn init(&self, app_config: AC) -> Result<ASP, AppStatePersistError> {
    fn init(
        &self,
        app_config: Self::AppConfig,
    ) -> Result<AppStateFilePersister, AppStatePersistError> {
        // create the directories, but no need to write the file, as there is only the default content
        // remove the last part, as this is the file
        let path = PathBuf::from(app_config.get_app_state_url());
        let directories = path
            .components()
            .take(path.components().count() - 1)
            .collect::<PathBuf>();
        create_dir_all(directories)
            .map_err(|io_err| AppStateFilePersisterError::IODirError(io_err, path.clone()))?;
        Ok(AppStateFilePersister {
            path: path.to_owned(),
        })
    }
}

/// Persists the application state to storage (a file).
/// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
impl AppStatePersister for AppStateFilePersister {
    type AppState = AppStateImpl;

    fn persist_app_state(&self, app_state: &Self::AppState) -> Result<(), AppStatePersistError> {
        //    fn save_app_state(app_state: &AppState, path: &Path) -> Result<(), io::Error> {
        trace!(
            "persisting app state:\n  {app_state:?}\n to {:?}",
            self.path
        );
        let serialized_app_state: Vec<u8> = bincode::serialize(app_state).expect(
            "bincode.serialzation itself should not result in an error, \
    unless the contract with serde is not respected!",
        );
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent).map_err(|error| {
                AppStateFilePersisterError::IOError(error, self.path.to_owned())
            })?;
        }
        File::create(&self.path)
            .and_then(|mut file| file.write_all(&serialized_app_state))
            .map_err(|ioerr| AppStateFilePersisterError::IOError(ioerr, self.path.to_owned()))?;
        trace!("Persisted app state to file: {:?}", self.path);
        Ok(())
    }

    // get the last persisted app state from a file, if any exists, otherwise creates a new app state
    // this function is only called once, in the initialization/app state constructor
    fn load_app_state(&self) -> Result<Self::AppState, AppStatePersistError> {
        trace!("loading the app state from {:?}", self.path);
        let loaded = std::fs::read(&self.path).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                AppStateFilePersisterError::FileNotFound(self.path.to_owned())
            } else {
                AppStateFilePersisterError::IOError(error, self.path.to_owned())
            }
        })?;
        let app_state: Self::AppState = bincode::deserialize(&loaded).map_err(|e| {
            AppStateFilePersisterError::DeserializationError(e, self.path.to_path_buf())
        })?;
        Ok(app_state)
    }
}

// #[cfg(test)]
// mod tests {
//     // don't execute the tests in parallel, as file access would lead to race conditions
//     use serial_test::serial;
//     use std::fs::{create_dir_all, File};
//     use std::io::ErrorKind as IoErrorKind;
//     use std::io::Write;
//     use std::path::PathBuf;
//     use std::sync::LazyLock;

//     use crate::application::api::contacts_api::ContactCommand;
//     use crate::application::app_config::AppConfig;
//     use crate::domain::common_model::CommonModel;
//     use crate::domain::contact::Contact;

//     use super::{load_app_state, save_app_state, AppState, AppStateLoadError};

//     /// Path to the temporary test directory
//     static TEST_FILE: LazyLock<PathBuf> =
//         LazyLock::new(|| PathBuf::from("delme/temptest/test_app_state_bin"));

//     /// Clean up the test directory after running tests
//     fn cleanup_test_file() {
//         if TEST_FILE.exists() {
//             let root_dir = TEST_FILE
//                 .components()
//                 .next()
//                 .expect("path includes directory names");
//             std::fs::remove_dir_all(root_dir).expect("Could not delete test file");
//         }
//     }
//     fn create_test_app_state() -> AppState {
//         let mut common_model = CommonModel::default();
//         let mut app_state = AppState::default();

//         let command = ContactCommand::AddContact {
//             contact: Contact {
//                 device_id: "test_device".to_string(),
//                 local_name: None,
//                 nickname: "Tester".to_string(),
//             },
//         };
//         command.process(&mut app_state.model.blocking_write());
//         app_state
//     }
//     fn assert_eq_app_states(left: &AppState, right: &AppState) {
//         assert_eq!(*left.model.blocking_read(), *right.model.blocking_read());
//     }

//     #[test]
//     #[serial]
//     fn read_existing_file() {
//         let original = create_test_app_state();
//         original
//             .persist(&TEST_FILE, &save_app_state)
//             .expect("App state not persisted");

//         let loaded = load_app_state(&TEST_FILE).expect("App state not loaded");

//         assert_eq_app_states(&original, &loaded);
//         cleanup_test_file();
//     }

//     #[test]
//     #[serial]
//     fn overwrite_existing_file() -> std::result::Result<(), std::io::Error> {
//         let original = create_test_app_state();
//         original
//             .persist(&TEST_FILE, &save_app_state)
//             .expect("App state not persisted");
//         assert!(&TEST_FILE.exists());

//         let mut changed = AppState::load_or_new(
//             &AppConfig {
//                 app_state_file_path: TEST_FILE.clone(),
//             },
//             &load_app_state,
//         )
//         .unwrap();
//         let change_command = ContactCommand::AddContact {
//             contact: Contact {
//                 device_id: "another test_device".to_string(),
//                 local_name: None,
//                 nickname: "Another Tester".to_string(),
//             },
//         };
//         change_command.process(&mut changed.model.blocking_write());
//         changed.persist(&TEST_FILE, &save_app_state);

//         let loaded = load_app_state(&TEST_FILE).unwrap();
//         assert_eq_app_states(&changed, &loaded);
//         cleanup_test_file();
//         Ok(())
//     }

//     #[test]
//     #[serial]
//     fn overwrite_corrupted_file() -> std::result::Result<(), std::io::Error> {
//         if let Some(parent) = TEST_FILE.parent() {
//             create_dir_all(parent).unwrap();
//         }
//         File::create(&*TEST_FILE)
//             .unwrap()
//             .write_all(b"corrupted")
//             .unwrap();

//         let changed = AppState::load_or_new(
//             &AppConfig {
//                 app_state_file_path: TEST_FILE.clone(),
//             },
//             &load_app_state,
//         )
//         .unwrap();
//         let _ = ContactCommand::AddContact {
//             contact: Contact {
//                 device_id: "another test_device".to_string(),
//                 local_name: None,
//                 nickname: "Another Tester".to_string(),
//             },
//         }
//         .process(&mut changed.model.blocking_write());

//         changed.persist(&TEST_FILE, &save_app_state).unwrap();

//         let loaded = load_app_state(&TEST_FILE).unwrap();
//         assert_eq_app_states(&changed, &loaded);
//         cleanup_test_file();
//         Ok(())
//     }

//     #[test]
//     #[serial]
//     fn load_corrupted_data() {
//         if let Some(parent) = TEST_FILE.parent() {
//             create_dir_all(parent).unwrap();
//         }
//         std::fs::write(&*TEST_FILE, "corrupted").unwrap();
//         let result = load_app_state(&TEST_FILE);
//         assert!(&result.is_err());
//         use bincode::ErrorKind;
//         assert!(matches!(
//             result,
//             Err(AppStateLoadError::DeserializationError(ref err, _))
//                 if {
//                    match &**err {
//                        ErrorKind::Io(ref inner_io_err) => {
//                            inner_io_err.kind() == IoErrorKind::UnexpectedEof
//                        },
//                        _ => false,
//                    }
//                 }
//         ));
//         cleanup_test_file();
//     }

//     #[test]
//     #[serial]
//     fn write_new_file() {
//         cleanup_test_file();
//         assert!(!TEST_FILE.exists());
//         let _ = create_test_app_state();

//         assert!(TEST_FILE.exists());
//         cleanup_test_file();
//     }

//     #[test]
//     #[serial]
//     fn test_load_not_existing_file() {
//         cleanup_test_file();
//         assert!(!TEST_FILE.exists());

//         let result = load_app_state(&TEST_FILE);

//         assert!(result.is_err());
//         assert!(matches!(result, Err(AppStateLoadError::FileNotFound(_))));
//         cleanup_test_file();
//     }

//     #[test]
//     #[serial]
//     fn cleanup() {
//         cleanup_test_file();
//     }
// }
