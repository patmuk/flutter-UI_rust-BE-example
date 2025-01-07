use std::{
    any::Any,
    fmt::Debug,
    fs::{create_dir_all, File},
    io::{self, ErrorKind as IoErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};

use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};

use crate::{
    application::api::lifecycle::{AppConfig, AppState, CqrsModelLock},
    domain::{
        todo_category::{TodoCategoryModel, TodoCategoryModelLock},
        todo_list::{TodoListModel, TodoListModelLock},
    },
};

use super::{api::lifecycle::AppStatePersister, app_config::AppConfigImpl};

/// holds the complete state of the app, as a global static variable
/// use `RustAutoOpaque<T>`, which is `Arc<RwLock<T>>`, on the fields,
/// which are written to concurrently, and which are exchanged with Dart,
/// but are too heavy to copy.
/// You could wrap the whole AppState in it,
/// but the finer granular the better parallelism you will get.
/// Just remember that you can not wrap children, if the parent is already wrapped.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppStateImpl {
    // flag, if persisting is needed
    dirty: AtomicBool,
    // We pretend that (parts of) the model are too hugh to performantly copy from Rust to Dart.
    // Thus we implement getters for the parts which need to be shown in the UI only.
    pub(crate) todo_list_model_lock: TodoListModelLock,
    pub(crate) todo_category_model_lock: TodoCategoryModelLock,
}

impl<AC: AppConfig> AppState<AC> for AppStateImpl {
    fn new(_app_config: &AC) -> Self {
        trace!("creating new app state");
        Self {
            todo_list_model_lock: TodoListModelLock::for_model(TodoListModel::default()),
            todo_category_model_lock: TodoCategoryModelLock::for_model(TodoCategoryModel::default()),
            dirty: AtomicBool::new(false),
        }
    }

    fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::SeqCst);
    }

    // /// Stores the app's state in a file.
    // fn persist_to_path(&self, path: &PathBuf) -> Result<(), io::Error> {
    //     if !self.dirty.load(Ordering::SeqCst) {
    //         trace!("app state os not dirty:\n  {self:?}");
    //     } else {
    //         trace!("persisting app state:\n  {self:?}\n to {:?}", path);
    //         trace!("App state is dirty, writing to file");
    //         let serialized_app_state: Vec<u8> = bincode::serialize(self).expect(
    //             "bincode.serialzation itself should not result in an error, \
    //         unless the contract with serde is not respected!",
    //         );
    //         if let Some(parent) = path.parent() {
    //             create_dir_all(parent)?;
    //         }
    //         File::create(path)?.write_all(&serialized_app_state)?;
    //         debug!("Persisted app state to file: {path:?}");
    //         self.dirty.store(false, Ordering::SeqCst);
    //     }
    //     Ok(())
    // }
    fn dirty_flag_value(&self) -> bool {
        self.dirty.load(Ordering::SeqCst)
    }
}

// impl AppStateImpl {
//     // get the last persisted app state from a file, if any exists, otherwise creates a new app state
//     // this function is only called once, in the initialization/app state constructor
//     fn load(path: &Path) -> Result<AppStateImpl, AppStateLoadError> {
//         trace!("loading the app state from {path:?}");
//         let loaded = std::fs::read(path)
//             .map_err(|error| AppStateLoadError::ReadFile(error, path.to_owned()))?;
//         let app_state: AppStateImpl = bincode::deserialize(&loaded)
//             .map_err(|e| AppStateLoadError::DeserializationError(e, path.to_path_buf()))?;
//         app_state.dirty.store(false, Ordering::SeqCst);
//         Ok(app_state)
//     }
// }

#[cfg(test)]
mod tests {
    // don't execute the tests in parallel, as file access would lead to race conditions
    use serial_test::serial;
    use std::fs::{create_dir_all, File};
    use std::io::ErrorKind as IoErrorKind;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use crate::application::api::lifecycle::AppState;

    use crate::application::app_state::{AppStateImpl, AppStateLoadError};

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
    fn create_test_app_state() -> AppStateImpl {
        let mut app_state = AppStateImpl::new(&TEST_FILE);
        mock_process_cqrs("Test setup todo".to_string(), &mut app_state)
            .expect("Could not persist the initial test state!");
        app_state
    }
    fn mock_process_cqrs(todo: String, app_state: &mut AppStateImpl) -> Result<(), std::io::Error> {
        let _ = app_state.todo_list_model_lock.command_add_todo(todo);
        app_state.mark_dirty();
        app_state.persist_to_path(&TEST_FILE)
    }
    fn assert_eq_app_states(left: &AppStateImpl, right: &AppStateImpl) {
        assert_eq!(
            *left.todo_list_model_lock.lock.blocking_read(),
            *right.todo_list_model_lock.lock.blocking_read()
        );
    }

    #[test]
    #[serial]
    fn read_existing_file() {
        let original = create_test_app_state();
        original
            .persist_to_path(&TEST_FILE)
            .expect("App state not persisted");

        let loaded = AppStateImpl::load(&TEST_FILE).expect("App state not loaded");

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

        let mut changed = AppStateImpl::new(&TEST_FILE);
        mock_process_cqrs("Changed todo".to_string(), &mut changed)?;

        let loaded = AppStateImpl::load(&TEST_FILE).unwrap();
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

        let mut changed = AppStateImpl::new(&TEST_FILE);
        mock_process_cqrs("Changed todo".to_string(), &mut changed)?;

        let loaded = AppStateImpl::load(&TEST_FILE).unwrap();
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
        let result = AppStateImpl::load(&TEST_FILE);
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

        let result = AppStateImpl::load(&TEST_FILE);

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
