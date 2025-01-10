use std::{
    fmt::Debug,
    sync::atomic::{AtomicBool, Ordering},
};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    application::api::lifecycle::{AppState, CqrsModelLock},
    domain::{
        todo_category::{TodoCategoryModel, TodoCategoryModelLock},
        todo_list::{TodoListModel, TodoListModelLock},
    },
};

use super::api::lifecycle::AppConfig;

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

impl AppState for AppStateImpl {
    fn new<AppConfigImpl: AppConfig>(_app_config: &AppConfigImpl) -> Self {
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

    fn dirty_flag_value(&self) -> bool {
        self.dirty.load(Ordering::SeqCst)
    }
}
