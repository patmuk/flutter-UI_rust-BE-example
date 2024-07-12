use std::{
    fs::{create_dir_all, File},
    io::{self, ErrorKind as IoErrorKind, Write},
    path::Path,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::{debug, error, info, trace};

use crate::domain::todo_list::TodoListModel;
// use crate::{api::lifecycle::AppConfig, ensure_logger_is_set_up, todo_list::TodoListModel};
use crate::{application::api::lifecycle::AppConfig, ensure_logger_is_set_up};

/// Stores the app's state in a file.
///
/// # Errors
///
/// This function will return an error if anything goes wrong
pub(crate) fn persist_app_state(app_state: &AppState, path: &Path) -> Result<(), io::Error> {
    trace!("persisting app state:\n  {app_state:?}\n to {:?}", path);

    let serialized_app_state: Vec<u8> =
        bincode::serialize(app_state).expect("bincode.serialzation itself should not result in an error, \
                                                    unless the contract with serde is not respected!");
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    File::create(path)?.write_all(&serialized_app_state)?;
    debug!("Persisted app state to file: {path:?}");
    Ok(())
}
// get the last persisted app state from a file, if any exists, otherwise creates a new app state
// this function is only called once, in the initialization/app state constructor
fn load(path: &Path) -> Result<AppState, AppStateLoadError> {
    debug!("loading the app state from {path:?}");
    let loaded =
        std::fs::read(path).map_err(|error| AppStateLoadError::ReadFile(error, path.to_owned()))?;
    let app_state = bincode::deserialize(&loaded)
        .map_err(|e| AppStateLoadError::DeserializationError(e, path.to_path_buf()))?;
    Ok(app_state)
}

// holds the complete state of the app, as a global static variable
#[derive(Default, Serialize, Deserialize, Debug)]
pub(crate) struct AppState {
    pub(crate) model: TodoListModel,
}

impl AppState {
    pub(crate) fn new(app_config: &AppConfig) -> Self {
        ensure_logger_is_set_up();
        debug!("creating the app state from persisted or default values");
        let app_state = match load(&app_config.app_state_file_path) {
            Err(AppStateLoadError::ReadFile(err, path)) if err.kind() == IoErrorKind::NotFound => {
                info!("No app state file found in {:?}, creating new state", &path);
                AppState::default()
            }
            Err(err) => {
                error!("Error reading file, creating a new state: {}", err);
                AppState::default()
            }
            Ok(loaded_app_state) => loaded_app_state,
        };
        info!(
            "Initialization finished, log level is {:?}",
            log::max_level()
        );
        app_state
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

    use crate::application::api::todo_list_api::Command;
    use crate::domain::todo_list::process_command_todo_list;

    use super::{AppState, AppStateLoadError};

    // Importing functions to test
    use super::{load, persist_app_state};

    /// Path to the temporary test directory
    use once_cell::sync::Lazy;
    static TEST_FILE: Lazy<PathBuf> =
        Lazy::new(|| PathBuf::from("delme/temptest/test_app_state_bin"));

    /// Clean up the test directory after running tests
    fn cleanup_test_file() {
        if TEST_FILE.exists() {
            std::fs::remove_dir_all(TEST_FILE.parent().unwrap()).unwrap();
        }
    }
    fn create_test_app_state() -> AppState {
        let mut app_state = AppState::default();
        process_command_todo_list(
            Command::AddTodo("Test setup todo".to_string()),
            &mut app_state.model,
        );
        app_state
    }
    fn assert_eq_app_states(left: &AppState, right: &AppState) {
        assert_eq!(&left.model, &right.model);
    }

    #[test]
    #[serial]
    fn read_existing_file() {
        let original = create_test_app_state();
        persist_app_state(&original, &TEST_FILE).unwrap();

        let loaded = load(&TEST_FILE).unwrap();

        assert_eq_app_states(&original, &loaded);
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn overwrite_existing_file() {
        let original = create_test_app_state();
        persist_app_state(&original, &TEST_FILE).unwrap();
        let mut changed = AppState::default();
        process_command_todo_list(
            Command::AddTodo("Changed todo".to_string()),
            &mut changed.model,
        );
        persist_app_state(&changed, &TEST_FILE).unwrap();

        let loaded = load(&TEST_FILE).unwrap();
        assert_eq_app_states(&changed, &loaded);
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn overwrite_corrupted_file() {
        if let Some(parent) = TEST_FILE.parent() {
            create_dir_all(parent).unwrap();
        }
        File::create(&*TEST_FILE)
            .unwrap()
            .write_all(b"corrupted")
            .unwrap();

        let mut changed = AppState::default();
        process_command_todo_list(
            Command::AddTodo("Changed todo".to_string()),
            &mut changed.model,
        );
        persist_app_state(&changed, &TEST_FILE).unwrap();

        let loaded = load(&TEST_FILE).unwrap();
        assert_eq_app_states(&changed, &loaded);
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn load_corrupted_data() {
        if let Some(parent) = TEST_FILE.parent() {
            create_dir_all(parent).unwrap();
        }
        std::fs::write(&*TEST_FILE, "corrupted").unwrap();
        let result = load(&TEST_FILE);
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
        let new_app_state = create_test_app_state();

        persist_app_state(&new_app_state, &TEST_FILE).unwrap();

        assert!(TEST_FILE.exists());
        cleanup_test_file();
    }

    #[test]
    #[serial]
    fn test_load_not_existing_file() {
        cleanup_test_file();
        assert!(!TEST_FILE.exists());

        let result = load(&TEST_FILE);

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
