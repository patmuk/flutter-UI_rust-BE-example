use bincode::ErrorKind;
use flutter_rust_bridge::frb;
use generate_cqrs_api_macro::generate_api;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::app_config::{self, AppConfigImpl};
use crate::application::app_state::{self, AppStateImpl};
use crate::infrastructure::app_state_db_persister::AppStateDBPersister;
use crate::infrastructure::app_state_file_persister::{
    AppStateFilePersister, AppStateFilePersisterError,
};
use crate::infrastructure::app_state_persistance_error::AppStatePersistError;

use std::io;
use std::io::ErrorKind as IoErrorKind;
use std::sync::OnceLock;

pub trait Lifecycle {
    /// loads the app's state, which can be io-heavy
    /// get the instance with get_singleton(). Create the initial singleton with UnInitilizedLifecycle::init()
    fn new<AC: AppConfig + std::fmt::Debug, ASP: AppStatePersister>(
        app_config: AC,
        // persister: ASP,
    ) -> Result<&'static Self, AppStatePersistError>;
    /// get the instance with get_singleton(). Create the initial singleton with Lifecycle::new()
    fn get_singleton() -> &'static Self;
    fn borrow_app_config<AC: AppConfig>(&self) -> &AC;
    fn borrow_app_state<AS: AppState>(&self) -> &AS;
    // fn borrow_app_state(&self) -> &AS;
    /// persist the app state to the previously stored location
    fn persist(&self) -> Result<(), AppStatePersistError>;
    fn shutdown(&self) -> Result<(), AppStatePersistError>;
}
// pub trait Lifecycle<AppConfig, AppState, AppStatePersister> {
//     /// loads the app's state, which can be io-heavy
//     /// get the instance with get_singleton(). Create the initial singleton with UnInitilizedLifecycle::init()
//     fn new(app_config: AC, persister: ASP) -> Result<&'static Self, AppStatePersistError>;
//     /// get the instance with get_singleton(). Create the initial singleton with Lifecycle::new()
//     fn get_singleton() -> &'static Self;
//     // fn borrow_app_config<AC: AppConfig>(&self) -> &AC;
//     fn borrow_app_config(&self) -> &AC;
//     // fn borrow_app_state<AS: AppState>(&self) -> &AS;
//     fn borrow_app_state(&self) -> &AS;
//     /// persist the app state to the previously stored location
//     fn persist(&self) -> Result<(), AppStatePersistError>;
//     fn shutdown(&self) -> Result<(), AppStatePersistError>;
// }

pub trait AppConfig: Default {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(url: Option<String>) -> Self;
    // app state storage location
    fn get_app_state_url(&self) -> &str;
}

pub trait AppState {
    fn new<AC: AppConfig>(app_config: &AC) -> Self;
    // where
    //     Self: Sized;
    fn dirty_flag_value(&self) -> bool;
    fn mark_dirty(&self);
}

pub trait AppStatePersister {
    /// prepares for persisting a new AppState. Not needed if the AppState is loaded!
    fn new<AC: AppConfig>(app_config: &AC) -> Result<Self, AppStatePersistError>
    where
        Self: Sized;
    /// Loads the application state.
    /// Returns a result with the `AppState` if successful or an `InfrastructureError` otherwise.
    fn load_app_state<AC: AppConfig, AS: AppState + Serialize + for<'a> Deserialize<'a>>(
        &self,
    ) -> Result<AS, AppStatePersistError>;

    /// Persists the application state to storage.
    /// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
    fn persist_app_state<AS: AppState + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug>(
        &self,
        state: &AS,
    ) -> Result<(), AppStatePersistError>;
}

// pub(crate) struct LifecycleImpl {
//     // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
//     pub(crate) app_config: AppConfig,
//     // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
//     pub(crate) app_state: AppState,
//     persister: AppStatePersister,
// }
pub(crate) struct LifecycleImpl {
    // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    pub(crate) app_config: AppConfigImpl,
    // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
    pub(crate) app_state: AppStateImpl,
    persister: AppStateFilePersister,
}

static SINGLETON: OnceLock<LifecycleImpl> = OnceLock::new();

/////////// tmp copy pasta from gened codeuse crate::domain::todo_category::*;
use crate::domain::todo_category::*;
use crate::domain::todo_list::*;
use log::debug;
use std::path::PathBuf;
pub(crate) trait CqrsModel:
    std::marker::Sized + Default + serde::Serialize + for<'de> serde::Deserialize<'de>
{
}
pub(crate) trait CqrsModelLock<CqrsModel>:
    std::marker::Sized + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn for_model(model: CqrsModel) -> Self;
}
pub trait Cqrs: std::fmt::Debug {
    fn process(self) -> Result<Vec<Effect>, ProcessingError>;
}
use crate::domain::todo_category::TodoCategoryProcessingError;
use crate::domain::todo_list::TodoListProcessingError;
#[derive(thiserror::Error, Debug)]
pub enum ProcessingError {
    #[error("Error during processing: {0}")]
    TodoListProcessingError(TodoListProcessingError),
    #[error("Error during processing: {0}")]
    TodoCategoryProcessingError(TodoCategoryProcessingError),
    #[error("Processing was fine, but state could not be persisted: {0}")]
    // NotPersisted(#[source] std::io::Error),
    NotPersisted(#[source] AppStatePersistError),
}
pub enum Effect {
    #[doc = " this indicates that the model has changed, so that the app\'s state should be persisted."]
    #[doc = " to avoid scanning the entire vec, this must be the first element."]
    TodoListModelRenderTodoList(TodoListModelLock),
    TodoListModelRenderTodoItem(TodoItem),
    TodoCategoryModelRenderTodoCategoryModel(TodoCategoryModelLock),
    TodoCategoryModelRenderTodoCategory(String),
    TodoCategoryModelRenderTodoList(TodoListModelLock),
    TodoCategoryModelUpdateTitel(String),
}
#[derive(Debug)]
pub enum TodoListModelQuery {
    GetAllTodos,
    GetTodo(usize),
}
#[derive(Debug)]
pub enum TodoListModelCommand {
    AddTodo(String),
    CleanList,
    RemoveTodo(usize),
}
impl Cqrs for TodoListModelQuery {
    fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let lifecycle = LifecycleImpl::get_singleton();
        let app_state = &lifecycle.app_state;
        let todo_list_model_lock = &app_state.todo_list_model_lock;
        let result = match self {
            TodoListModelQuery::GetAllTodos => todo_list_model_lock.query_get_all_todos(),
            TodoListModelQuery::GetTodo(todo_pos) => todo_list_model_lock.query_get_todo(todo_pos),
        }
        .map_err(ProcessingError::TodoListProcessingError)?;
        Ok(result
            .into_iter()
            .map(|effect| match effect {
                TodoListEffect::RenderTodoList(todo_list_model_lock) => {
                    Effect::TodoListModelRenderTodoList(todo_list_model_lock)
                }
                TodoListEffect::RenderTodoItem(todo_item) => {
                    Effect::TodoListModelRenderTodoItem(todo_item)
                }
            })
            .collect())
    }
}
impl Cqrs for TodoListModelCommand {
    fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let lifecycle = LifecycleImpl::get_singleton();
        let app_state = &lifecycle.app_state;
        let todo_list_model_lock = &app_state.todo_list_model_lock;
        let (state_changed, result) = match self {
            TodoListModelCommand::AddTodo(todo) => todo_list_model_lock.command_add_todo(todo),
            TodoListModelCommand::CleanList => todo_list_model_lock.command_clean_list(),
            TodoListModelCommand::RemoveTodo(todo_pos) => {
                todo_list_model_lock.command_remove_todo(todo_pos)
            }
        }
        .map_err(ProcessingError::TodoListProcessingError)?;
        if state_changed {
            app_state.mark_dirty();
            lifecycle.persist().map_err(ProcessingError::NotPersisted)?;
        }
        Ok(result
            .into_iter()
            .map(|effect| match effect {
                TodoListEffect::RenderTodoList(todo_list_model_lock) => {
                    Effect::TodoListModelRenderTodoList(todo_list_model_lock)
                }
                TodoListEffect::RenderTodoItem(todo_item) => {
                    Effect::TodoListModelRenderTodoItem(todo_item)
                }
            })
            .collect())
    }
}
#[derive(Debug)]
pub enum TodoCategoryModelQuery {
    GetTodoCategory(bool),
    GetTodoCategoryModel,
    GetTodoList,
}
#[derive(Debug)]
pub enum TodoCategoryModelCommand {
    SetTodoList(TodoListModelLock),
    UpdateTitle(String),
}
impl Cqrs for TodoCategoryModelQuery {
    fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let lifecycle = LifecycleImpl::get_singleton();
        let app_state = &lifecycle.app_state;
        let todo_category_model_lock = &app_state.todo_category_model_lock;
        let result = match self {
            TodoCategoryModelQuery::GetTodoCategory(get_model) => {
                todo_category_model_lock.get_todo_category(get_model)
            }
            TodoCategoryModelQuery::GetTodoCategoryModel => {
                todo_category_model_lock.get_todo_category_model()
            }
            TodoCategoryModelQuery::GetTodoList => todo_category_model_lock.get_todo_list(),
        }
        .map_err(ProcessingError::TodoCategoryProcessingError)?;
        Ok(result
            .into_iter()
            .map(|effect| match effect {
                TodoCategoryEffect::RenderTodoCategoryModel(todo_category_model_lock) => {
                    Effect::TodoCategoryModelRenderTodoCategoryModel(todo_category_model_lock)
                }
                TodoCategoryEffect::RenderTodoCategory(string) => {
                    Effect::TodoCategoryModelRenderTodoCategory(string)
                }
                TodoCategoryEffect::RenderTodoList(todo_list_model_lock) => {
                    Effect::TodoCategoryModelRenderTodoList(todo_list_model_lock)
                }
                TodoCategoryEffect::UpdateTitel(string) => {
                    Effect::TodoCategoryModelUpdateTitel(string)
                }
            })
            .collect())
    }
}
impl Cqrs for TodoCategoryModelCommand {
    fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let lifecycle = LifecycleImpl::get_singleton();
        let app_state = &lifecycle.app_state;
        let todo_category_model_lock = &app_state.todo_category_model_lock;
        let (state_changed, result) = match self {
            TodoCategoryModelCommand::SetTodoList(todo_list_lock) => {
                todo_category_model_lock.set_todo_list(todo_list_lock)
            }
            TodoCategoryModelCommand::UpdateTitle(title) => {
                todo_category_model_lock.update_title(title)
            }
        }
        .map_err(ProcessingError::TodoCategoryProcessingError)?;
        if state_changed {
            app_state.mark_dirty();
            lifecycle.persist().map_err(ProcessingError::NotPersisted)?;
        }
        Ok(result
            .into_iter()
            .map(|effect| match effect {
                TodoCategoryEffect::RenderTodoCategoryModel(todo_category_model_lock) => {
                    Effect::TodoCategoryModelRenderTodoCategoryModel(todo_category_model_lock)
                }
                TodoCategoryEffect::RenderTodoCategory(string) => {
                    Effect::TodoCategoryModelRenderTodoCategory(string)
                }
                TodoCategoryEffect::RenderTodoList(todo_list_model_lock) => {
                    Effect::TodoCategoryModelRenderTodoList(todo_list_model_lock)
                }
                TodoCategoryEffect::UpdateTitel(string) => {
                    Effect::TodoCategoryModelUpdateTitel(string)
                }
            })
            .collect())
    }
}
// #[generate_api(
//     "app_core/src/domain/todo_list.rs",
//     "app_core/src/domain/todo_category.rs"
// )]
impl Lifecycle for LifecycleImpl {
    // impl Lifecycle<AppConfigImpl, AppStateImpl, AppStateFilePersister> for LifecycleImpl {
    fn new<AC: AppConfig + std::fmt::Debug, ASP: AppStatePersister>(
        // fn new<AC: AppConfig + std::fmt::Debug, AppStateFilePersister>(
        app_config: AC,
        // persister: ASP,
        // persister: AppStateFilePersister,
    ) -> Result<&'static Self, AppStatePersistError> {
        info!("Initializing app with config: {:?}", &app_config);
        let persister = AppStateFilePersister::new(&app_config)?;
        // calling init() the first time creates the singleton. (Although self is consumed, there migth be multiple instances of self.)
        // not using SINGLETON.get_or_init() so we can propergate the AppStatePersistError
        let result = match SINGLETON.get() {
            Some(instance) => Ok(instance),
            None => {
                let app_state = match persister.load_app_state::<AppConfigImpl, AppStateImpl>() {
                    Ok(app_state) => app_state,
                    Err(AppStatePersistError::DiskError(disk_err)) => match disk_err {
                        AppStateFilePersisterError::FileNotFound(file_path)
                        // todo match on IO-FileNotFound or avoid this error type duplication
                        // | AppStateFilePersisterError::IOError(io_Error, file_path)
                        //     if io_Error.kind() == IoErrorKind::NotFound
                            =>
                        {
                            info!(
                                "No app state file found in {:?}, creating new state there.",
                                &file_path
                            );
                            let app_state = AppState::new(&app_config);
                            persister.persist_app_state(&app_state)?;
                            app_state
                        }
                        _ => return Err(AppStatePersistError::DiskError(disk_err)),
                    },
                    Err(e) => return Err(e),
                };
                let app_config_impl =
                    AppConfigImpl::new(Some(app_config.get_app_state_url().to_string()));
                let _ = SINGLETON.set(LifecycleImpl {
                    app_config: app_config_impl,
                    app_state,
                    persister,
                });
                Ok(SINGLETON
                    .get()
                    .expect("Impossible error - content has just been set!"))
            }
        };
        info!(
            "Initialization finished, log level is {:?}",
            log::max_level()
        );
        result
    }

    fn get_singleton() -> &'static Self {
        SINGLETON.get().expect(
            "Lifecycle: should been initialized with Lifecycle::new(AppConfig, AppStatePersister)!",
        )
    }

    // fn borrow_app_state(&self) -> &AppStateImpl {
    //     &self.app_state
    // }
    fn borrow_app_state<AS: AppState>(&self) -> &AS {
        unsafe { std::mem::transmute(&self.app_state) }
        // &self.app_state
    }

    fn borrow_app_config<AC: AppConfig>(&self) -> &AC {
        // fn borrow_app_config(&self) -> &AppConfigImpl {
        unsafe { std::mem::transmute(&self.app_config) }
        // &self.app_config
    }

    /// persist the app state to the previously stored location
    fn persist(&self) -> Result<(), AppStatePersistError> {
        self.persister.persist_app_state(&self.app_state)
    }

    fn shutdown(&self) -> Result<(), AppStatePersistError> {
        info!("shutting down the app");
        // blocks on the Locks of inner fields
        // TODO implent timeout and throw an error?
        self.persist()
    }
}
