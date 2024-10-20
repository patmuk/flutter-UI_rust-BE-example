use std::{
    fmt::Debug,
    fs::{create_dir_all, File},
    io::{self, ErrorKind as IoErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};

use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};

use crate::application::{
    api::lifecycle::{AppConfig, Lifecycle},
    bridge::frb_generated::RustAutoOpaque,
};
use crate::domain::todo_list::TodoListModel;

/// holds the complete state of the app, as a global static variable
/// use `RustAutoOpaque<T>`, which is `Arc<RwLock<T>>`, on the fields,
/// which are written to concurrently, and which are exchanged with Dart,
/// but are too heavy to copy.
/// You could wrap the whole AppState in it,
/// but the finer granular the better parallelism you will get.
/// Just remember that you can not wrap children, if the parent is already wrapped.
#[derive(Debug)]
pub(crate) struct AppState {
    // We pretend that (parts of) the model are too hugh to performantly copy from Rust to Dart.
    // Thus we implement getters for the parts which need to be shown in the UI only.
    pub(crate) model: RustAutoOpaque<TodoListModel>,
    // flag, if writing to disc is needed
    dirty: AtomicBool,
}

impl Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the model, the dirty flag is always false after loading
        self.model.blocking_read().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AppState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the model and dirty flag. The dirty flag is always false after loading
        let model = TodoListModel::deserialize(deserializer)?;
        // let model = RustAutoOpaque::<TodoListModel>::deserialize(deserializer)?;
        Ok(AppState {
            model: RustAutoOpaque::new(model),
            dirty: AtomicBool::new(false),
        })
    }
}

impl AppState {
    // called by the lifecycle initialization. Get the app state over the lifecycle singleton.
    pub(crate) fn load_or_new(app_config: &AppConfig) -> Self {
        debug!("creating the app state from persisted or default values");
        let app_state = match AppState::load(&app_config.app_state_file_path) {
            Err(AppStateLoadError::ReadFile(err, path)) if err.kind() == IoErrorKind::NotFound => {
                info!(
                    "No app state file found in {:?}, creating new state there.",
                    &path
                );
                AppState::new(&app_config.app_state_file_path)
            }
            Err(err) => {
                panic!(
                    "Error loading app state from file {:?}: {}",
                    &app_config.app_state_file_path, err
                );
                // TODO better handling
                // error!("Error reading file, creating a new state: {}", err);
                // AppState::new(None)
            }
            Ok(loaded_app_state) => loaded_app_state,
        };
        info!(
            "Initialization finished, log level is {:?}",
            log::max_level()
        );
        app_state
    }
    fn new(path: &PathBuf) -> Self {
        trace!("creating new app state");
        // create the directories, but no need to write the file, as there is only the default content
        // remove the last part, as this is the file
        let directories = path
            .components()
            .take(path.components().count() - 1)
            .collect::<PathBuf>();
        create_dir_all(directories).unwrap_or_else(|_| {
            panic!(
                "failed to create directories for the app's state persistance in {:?}",
                &path
            )
        });
        AppState {
            model: RustAutoOpaque::new(TodoListModel::default()),
            dirty: AtomicBool::new(false),
        }
    }
    #[cfg(test)]
    pub(crate) fn from_model(model: &RustAutoOpaque<TodoListModel>) -> Self {
        Self {
            model: model.clone(),
            dirty: AtomicBool::new(false),
        }
    }
    // get the last persisted app state from a file, if any exists, otherwise creates a new app state
    // this function is only called once, in the initialization/app state constructor
    fn load(path: &Path) -> Result<AppState, AppStateLoadError> {
        trace!("loading the app state from {path:?}");
        let loaded = std::fs::read(path)
            .map_err(|error| AppStateLoadError::ReadFile(error, path.to_owned()))?;
        let app_state: AppState = bincode::deserialize(&loaded)
            .map_err(|e| AppStateLoadError::DeserializationError(e, path.to_path_buf()))?;
        app_state.dirty.store(false, Ordering::SeqCst);
        Ok(app_state)
    }
    /// persist the app state to the previously stored location
    pub(crate) fn persist(&self) -> Result<(), io::Error> {
        self.persist_to_path(&Lifecycle::get().app_config.app_state_file_path)
    }
    /// Stores the app's state in a file.
    pub(crate) fn persist_to_path(&self, path: &Path) -> Result<(), io::Error> {
        if !self.dirty.load(Ordering::SeqCst) {
            trace!("app state os not dirty:\n  {self:?}");
        } else {
            trace!("persisting app state:\n  {self:?}\n to {:?}", path);
            trace!("App state is dirty, writing to file");
            let serialized_app_state: Vec<u8> =
                bincode::serialize(self).expect("bincode.serialzation itself should not result in an error, \
                                                    unless the contract with serde is not respected!");
            if let Some(parent) = path.parent() {
                create_dir_all(parent)?;
            }
            File::create(path)?.write_all(&serialized_app_state)?;
            debug!("Persisted app state to file: {path:?}");
            self.dirty.store(false, Ordering::SeqCst);
        }
        Ok(())
    }
    pub(crate) fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::SeqCst);
    }
}

// handle errors as suggested by https://kazlauskas.me/entries/errors
#[derive(thiserror::Error, Debug)]
pub enum AppStateLoadError {
    #[error("Cannot read file from path: {1}")]
    ReadFile(#[source] io::Error, PathBuf),
    #[error("could not understand (=deserialize) the file {1}. Maybe it's content got corrupted?")]
    DeserializationError(#[source] bincode::Error, PathBuf),
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

    use crate::application::api::processing::Cqrs;

    use super::{AppState, AppStateLoadError};

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
    fn create_test_app_state() -> AppState {
        let mut app_state = AppState::new(&TEST_FILE);
        mock_process_cqrs(
            Cqrs::TodoCommandAddTodo("Test setup todo".to_string()),
            &mut app_state,
        )
        .expect("Could not persist the initial test state!");
        app_state
    }
    fn mock_process_cqrs(cqrs: Cqrs, app_state: &mut AppState) -> Result<(), std::io::Error> {
        cqrs.process_with_app_state(app_state).unwrap();
        app_state.mark_dirty();
        app_state.persist_to_path(&TEST_FILE)
    }
    fn assert_eq_app_states(left: &AppState, right: &AppState) {
        assert_eq!(*left.model.blocking_read(), *right.model.blocking_read());
    }

    #[test]
    #[serial]
    fn read_existing_file() {
        let original = create_test_app_state();
        original
            .persist_to_path(&TEST_FILE)
            .expect("App state not persisted");

        let loaded = AppState::load(&TEST_FILE).expect("App state not loaded");

        assert_eq_app_states(&original, &loaded);
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn overwrite_existing_file() -> std::result::Result<(), std::io::Error> {
        let original = create_test_app_state();
        original
            .persist_to_path(&TEST_FILE)
            .expect("App state not persisted");
        assert!(&TEST_FILE.exists());

        let mut changed = AppState::new(&TEST_FILE);
        mock_process_cqrs(
            Cqrs::TodoCommandAddTodo("Changed todo".to_string()),
            &mut changed,
        )?;

        let loaded = AppState::load(&TEST_FILE).unwrap();
        assert_eq_app_states(&changed, &loaded);
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

        let mut changed = AppState::new(&TEST_FILE);
        mock_process_cqrs(
            Cqrs::TodoCommandAddTodo("Changed todo".to_string()),
            &mut changed,
        )?;

        let loaded = AppState::load(&TEST_FILE).unwrap();
        assert_eq_app_states(&changed, &loaded);
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
        let result = AppState::load(&TEST_FILE);
        assert!(&result.is_err());
        use bincode::ErrorKind;
        assert!(matches!(
            result,
            Err(AppStateLoadError::DeserializationError(ref err, _))
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

        let result = AppState::load(&TEST_FILE);

        assert!(result.is_err());
        assert!(
            matches!(result, Err(AppStateLoadError::ReadFile(ref err, _)) if err.kind() == IoErrorKind::NotFound)
        );
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn cleanup() {
        cleanup_test_file();
    }
}
