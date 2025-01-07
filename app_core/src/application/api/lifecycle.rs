use bincode::ErrorKind;
use generate_cqrs_api_macro::generate_api;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::app_config::AppConfigImpl;
use crate::application::app_state::{self, AppStateImpl};
use crate::infrastructure::app_state_file_persister::{
    AppStateFilePersister, AppStateFilePersisterError,
};
use crate::infrastructure::app_state_persistance_error::AppStatePersistError;

use std::io;
use std::io::ErrorKind as IoErrorKind;
use std::sync::OnceLock;

pub trait UnInitializedLifecycle<AC: AppConfig> {
    type LifecycleImplementation;
    fn new(app_config: AC) -> Self;
    /// the app state can be re-set as long as init() has't been called
    fn reset_app_config(&mut self, app_config: AC) -> &mut Self;
    fn app_config(&self) -> &AC;
    /// loads the app's state, which can be io-heavy
    /// get the instance with get_singleton(). Create the initial singleton with UnInitilizedLifecycle::init()
    fn init<P: AppStatePersister<AC, AS>, AS: AppState<AC>, LifecycleImplementation>(
        self,
        persister: P,
    ) -> Result<&'static LifecycleImplementation, AppStatePersistError>;
}
pub trait Lifecycle<AC: AppConfig, AS: AppState<AC>> {
    /// get the instance with get_singleton(). Create the initial singleton with UnInitilizedLifecycle::init()
    fn get_singleton() -> &'static Self
    where
        Self: Sized;
    fn app_config(&self) -> &AC;
    fn app_state(&self) -> &AS;
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

pub trait AppStatePersister<AC: AppConfig, AS: AppState<AC>> {
    /// Loads the application state.
    /// Returns a result with the `AppState` if successful or an `InfrastructureError` otherwise.
    fn load_app_state(&self) -> Result<AS, AppStatePersistError>;

    /// Persists the application state to storage.
    /// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
    fn persist_app_state(&self, state: &AS) -> Result<(), AppStatePersistError>;
}

static SINGLETON: OnceLock<LifecycleImpl<AppConfigImpl, AppStateImpl, AppStateFilePersister>> =
    OnceLock::new();

struct UnInitializedLifeCycleImpl<AC: AppConfig> {
    app_config: AC,
}

struct LifecycleImpl<AC: AppConfig, AS: AppState<AC>, P: AppStatePersister<AC, AS>> {
    // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    pub(crate) app_config: AC,
    // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
    pub(crate) app_state: AS,
    persister: P,
}

impl<AC: AppConfig> UnInitializedLifecycle<AC> for UnInitializedLifeCycleImpl<AC> {
    type LifecycleImplementation =
        LifecycleImpl<AppConfigImpl, AppStateImpl, AppStateFilePersister>;

    fn new(app_config: AC) -> Self {
        Self { app_config }
    }

    fn reset_app_config(&mut self, app_config: AC) -> &mut Self {
        self.app_config = app_config;
        self
    }

    fn app_config(&self) -> &AC {
        &self.app_config
    }

    fn init<P: AppStatePersister<AC, AS>, AS: AppState<AC>, LifecycleImplementation>(
        self,
        persister: P,
        // ) -> Result<&'static LifecycleImplementation, AppStatePersistError> {
    ) -> Result<&'static LifecycleImplementation, AppStatePersistError> {
        info!("Initializing app with config: {:?}", &self.app_config);
        // calling init() the first time creates the singleton. (Although self is consumed, there migth be multiple instances of self.)
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
                            // let app_state = AppStateImpl::new(&self.app_config);
                            let app_state = AS::new(&self.app_config);
                            persister.persist_app_state(&app_state)?;
                            app_state
                        }
                        _ => return Err(AppStatePersistError::DiskError(disk_err)),
                    },
                    Err(e) => return Err(e),
                };
                let lifecycle = LifecycleImpl {
                    app_config: self.app_config,
                    app_state,
                    persister,
                };
                SINGLETON
                    .set(lifecycle)
                    .expect("Lifecycle has already been initialized!");
                Ok(&lifecycle)
            }
        };
        info!(
            "Initialization finished, log level is {:?}",
            log::max_level()
        );
        result
    }
}

#[generate_api(
    "app_core/src/domain/todo_list.rs",
    "app_core/src/domain/todo_category.rs"
)]
impl<AC: AppConfig, AS: AppState<AC>, ASP: AppStatePersister<AC, AS>> Lifecycle<AC, AS>
    for LifecycleImpl<AC, AS, ASP>
{
    fn get_singleton() -> &'static Self {
        SINGLETON
            .get()
            .expect("Lifecycle: should been initialized with UnInitializedLifecycle::init()!")
    }

    fn app_state(&self) -> &AS {
        &self.app_state
    }

    fn app_config(&self) -> &AC {
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
