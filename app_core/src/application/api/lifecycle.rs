use flutter_rust_bridge::frb;
use log::{debug, trace};

use crate::application::app_state::{self, AppState};
pub use crate::application::processing_errors::ProcessingError;
pub use crate::domain::effects::Effect;
use crate::domain::todo_list::TodoListModel;
pub use crate::utils::cqrs_traits::Cqrs as CqrsTrait;
use crate::utils::cqrs_traits::CqrsModel;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

static SINGLETON: OnceLock<Lifecycle> = OnceLock::new();

///////////
//// processing here to see codegen results
//////////
//TODO replace with macro_rules!([TodoComand, TodoQuery])
pub enum Cqrs {
    TodoCommand(TodoCommand),
    TodoQuery(TodoQuery),
}
pub enum TodoCommand {
    AddTodo(String),
    RemoveTodo(usize),
    CleanList,
}
pub enum TodoQuery {
    AllTodos,
}

// pub trait Wrapping {
//     fn wrap(self) -> WrappedCqrs;
//     fn process(self, app_state: &AppState) -> Result<Vec<Effect>, ProcessingError>;
// }

// impl Wrapping for TodoCommand {
impl Cqrs {
    fn process(self, app_state: &AppState) -> Result<Vec<Effect>, ProcessingError> {
        let result = match self {
            Cqrs::TodoCommand(todo_command) => match todo_command {
                TodoCommand::AddTodo(todo) => TodoListModel::add_todo(app_state, todo),
                TodoCommand::RemoveTodo(todo_pos) => {
                    TodoListModel::remove_todo(app_state, todo_pos)
                }
                TodoCommand::CleanList => TodoListModel::clean_list(app_state),
            },
            Cqrs::TodoQuery(todo_query) => match todo_query {
                TodoQuery::AllTodos => TodoListModel::all_todos(app_state),
            },
        };
        //persist the state, but only if dirty
        app_state.persist(&Lifecycle::get().app_config.app_state_file_path);
    }
}

pub fn process_cqrs(cqrs: Cqrs) -> Result<Vec<Effect>, ProcessingError> {
    cqrs.process(&AppState::get())
}
// fn inner_process_cqrs(cqrs: impl Cqrs) -> Result<Vec<Effect>, ProcessingError> {
//     let lifecycle = Lifecycle::get();
//     let is_command = cqrs.is_command();
//     if is_command {
//         debug!("Processing cqrs_command: {:?}", cqrs);
//     } else {
//         debug!("Processing cqrs_query: {:?}", cqrs);
//     }
//     let effects = cqrs.process(&lifecycle.app_state);
//     debug!(
//         "Processed cqrs, new model {:?}",
//         lifecycle.app_state.model.blocking_read()
//     );
//     debug!("got the effects {:?}", effects);
//     if is_command {
//         lifecycle.app_state.mark_dirty();
//         // persist change to not miss it
//         lifecycle
//             .app_state
//             .persist(&lifecycle.app_config.app_state_file_path)
//             .unwrap(); // TODO convert to own error
//                        // Ok::<_, dyn ProcessingError>(effects)
//     }
//     effects
//     // }
// }

/////////////

// #[frb(opaque)]
pub struct Lifecycle {
    // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    pub(crate) app_config: AppConfig,
    // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
    pub(crate) app_state: AppState,
}

impl Lifecycle {
    // to avoid an illegal state (app state not loaded) we do the setup and init in one go
    pub fn new(path: Option<String>) -> &'static Self {
        OnceLock::get_or_init(&SINGLETON, || {
            let app_config = Lifecycle::setup(path);
            let app_state: AppState = Lifecycle::init(&app_config);

            Lifecycle {
                app_config,
                app_state,
            }
        })
    }

    pub fn get() -> &'static Self {
        OnceLock::get(&SINGLETON).expect("Lifecycle: should been initialized with ::new()!")
    }

    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn setup(path: Option<String>) -> AppConfig {
        match path {
            Some(path) => {
                trace!(
                "Overwriting default setup:\n  - setting the app_state_storage_path to {path:?}"
            );
                AppConfig {
                    app_state_file_path: PathBuf::from(path),
                }
            }
            None => {
                debug!("Using default path in setup");
                AppConfig::default()
            }
        }
    }

    /// call to initialize the app.
    /// loads the app's state, which can be io-heavy
    fn init(app_config: &AppConfig) -> AppState {
        debug!("Initializing app with config: {:?}", app_config);
        AppState::load_or_new(app_config)
    }

    pub fn shutdown(&self) -> Result<(), io::Error> {
        debug!("shutting down the app");
        // blocks on the Locks of inner fields
        // TODO implent timeout and throw an error?
        self.app_state.persist(&self.app_config.app_state_file_path)
    }
    // pub fn process_cqrs<C: Cqrs>(&self, cqrs: C) -> Result<Vec<Effect>, ProcessingError> {
    //     let is_command = cqrs.is_command();
    //     if is_command {
    //         debug!("Processing cqrs_command: {:?}", cqrs);
    //     } else {
    //         debug!("Processing cqrs_query: {:?}", cqrs);
    //     }
    //     let effects = cqrs.process(&self.app_state);
    //     debug!(
    //         "Processed cqrs, new model {:?}",
    //         self.app_state.model.blocking_read()
    //     );
    //     debug!("got the effects {:?}", effects);
    //     if is_command {
    //         self.app_state.mark_dirty();
    //         // persist change to not miss it
    //         self.app_state
    //             .persist(&self.app_config.app_state_file_path)
    //             .unwrap(); // TODO convert to own error
    //                        // Ok::<_, dyn ProcessingError>(effects)
    //     }
    //     effects
    // }
}
// app state storage location
#[derive(Debug)]
pub struct AppConfig {
    pub app_state_file_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            app_state_file_path: PathBuf::from("./app_state_model.bin"),
        }
    }
}
