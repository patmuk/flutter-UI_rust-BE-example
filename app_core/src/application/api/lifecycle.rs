use bincode::ErrorKind;
use generate_cqrs_api_macro::generate_api;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::app_config::{self, AppConfigImpl};
use crate::application::app_state::{self, AppStateImpl};
use crate::infrastructure::app_state_file_persister::{
    AppStateFilePersister, AppStateFilePersisterError,
};
use crate::infrastructure::app_state_persistance_error::AppStatePersistError;

use std::io;
use std::io::ErrorKind as IoErrorKind;
use std::sync::OnceLock;

pub trait Lifecycle {
    type AppConfig;
    type AppState;
    type AppStatePersister;
    type LifecycleSingleton;
    //     /// loads the app's state, which can be io-heavy
    //     /// get the instance with get_singleton(). Create the initial singleton with UnInitilizedLifecycle::init()
    fn new(
        app_config: Self::AppConfig,
        persister: Self::AppStatePersister,
    ) -> Result<&'static Self::LifecycleSingleton, AppStatePersistError>;
    // fn new<P: AppStatePersister<AC, AS>>(
    //     app_config: AC,
    //     persister: P,
    // ) -> Result<&'static Self::LifecycleSingleton, AppStatePersistError>;
    /// get the instance with get_singleton(). Create the initial singleton with Lifecycle::new()
    fn get_singleton() -> &'static Self
    where
        Self: Sized;
    fn app_config(&self) -> &Self::AppConfig;
    fn app_state(&self) -> &Self::AppState;
    /// persist the app state to the previously stored location
    fn persist(&self) -> Result<(), AppStatePersistError>;
    fn shutdown(&self) -> Result<(), AppStatePersistError>;
}

pub trait AppConfig: Default + std::fmt::Debug {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(path: Option<String>) -> Self;
    // app state storage location
    fn get_app_state_url(&self) -> &std::path::PathBuf;
}
pub trait AppState<AC: AppConfig>: Serialize + for<'a> Deserialize<'a> {
    fn new(app_config: &AC) -> Self
    where
        Self: Sized;
    fn dirty_flag_value(&self) -> bool;
    fn mark_dirty(&self);
}
pub trait UnititializedAppStatePersister<AC: AppConfig> {
    type AppStatePersisterImplementation;
    /// prepares for persisting a new AppState. Not needed if the AppState is loaded!
    fn init(
        &self,
        app_config: AC,
    ) -> Result<Self::AppStatePersisterImplementation, AppStatePersistError>;
}

pub trait AppStatePersister {
    type AppState;
    /// Loads the application state.
    /// Returns a result with the `AppState` if successful or an `InfrastructureError` otherwise.
    fn load_app_state(&self) -> Result<Self::AppState, AppStatePersistError>;

    /// Persists the application state to storage.
    /// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
    fn persist_app_state(&self, state: &Self::AppState) -> Result<(), AppStatePersistError>;
}

static SINGLETON: OnceLock<LifecycleImpl> = OnceLock::new();

struct UnInitializedLifeCycleImpl<AC: AppConfig> {
    app_config: AC,
}

struct LifecycleImpl {
    // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    pub(crate) app_config: AppConfigImpl,
    // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
    pub(crate) app_state: AppStateImpl,
    persister: AppStateFilePersister,
}
// struct LifecycleImpl<AC: AppConfig, AS: AppState<AC>, P: AppStatePersister> {
//     // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
//     pub(crate) app_config: AC,
//     // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
//     pub(crate) app_state: AS,
//     persister: P,
// }

// struct LifecycleSingleton {
//     instance: LifecycleImpl<AppConfigImpl, AppStateImpl, AppStateFilePersister>,
// }

#[generate_api(
    "app_core/src/domain/todo_list.rs",
    "app_core/src/domain/todo_category.rs"
)]
impl Lifecycle for LifecycleImpl {
    type AppConfig = AppConfigImpl;
    type AppState = AppStateImpl;
    type AppStatePersister = AppStateFilePersister;
    type LifecycleSingleton = LifecycleImpl;

    fn new(
        app_config: Self::AppConfig,
        persister: Self::AppStatePersister,
    ) -> Result<&'static Self::LifecycleSingleton, AppStatePersistError> {
        // as this is static, it is executed one time only! So there is only one OnceLock instance.
        // static SINGLETON: OnceLock<Self::LifecycleSingleton> = OnceLock::new();
        static SINGLETON: OnceLock<LifecycleImpl> = OnceLock::new();

        info!("Initializing app with config: {:?}", &app_config);
        // calling init() the first time creates the singleton. (Although self is consumed, there migth be multiple instances of self.)
        // not using SINGLETON.get_or_init() so we can propergate the AppStatePersistError
        let result = match SINGLETON.get() {
            Some(instance) => Ok(instance),
            None => {
                let app_state = match persister.load_app_state() {
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
                            let app_state = Self::AppState::new(&app_config);
                            persister.persist_app_state(&app_state)?;
                            app_state
                        }
                        _ => return Err(AppStatePersistError::DiskError(disk_err)),
                    },
                    Err(e) => return Err(e),
                };
                // let lifecycle_singleton = LifecycleSingleton {
                //     instance: LifecycleImpl {
                //         app_config,
                //         app_state,
                //         persister,
                //     },
                // };
                let lifecycle_singleton = LifecycleImpl {
                    app_config,
                    app_state,
                    persister,
                };
                SINGLETON.set(lifecycle_singleton);
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

    fn get_singleton() -> &'static Self::LifecycleSingleton {
        SINGLETON
            .get()
            .expect("Lifecycle: should been initialized with UnInitializedLifecycle::init()!")
    }

    fn app_state(&self) -> &Self::AppState {
        &self.app_state
    }

    fn app_config(&self) -> &Self::AppConfig {
        &self.app_config
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
