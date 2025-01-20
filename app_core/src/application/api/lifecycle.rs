use generate_cqrs_api_macro::generate_api;
use log::info;

pub use crate::application::app_config::AppConfigImpl;
use crate::application::app_state::AppStateImpl;
use crate::infrastructure::app_state_file_persister::{
    AppStateFilePersister, AppStateFilePersisterError,
};

use std::sync::OnceLock;

pub struct LifecycleImpl {
    // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    pub app_config: AppConfigImpl,
    // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
    pub(crate) app_state: AppStateImpl,
    pub(crate) persister: AppStateFilePersister,
}

static SINGLETON: OnceLock<LifecycleImpl> = OnceLock::new();

#[generate_api(
    "app_core/src/domain/todo_list.rs",
    "app_core/src/domain/todo_category.rs"
)]
/// frb doesn't support generics. If needed implement them using enums or the enum_dispatch crate.
impl Lifecycle for LifecycleImpl {
    type Error = AppStateFilePersisterError;
    fn initialise_with_app_config<AC: AppConfig + std::fmt::Debug>(
        app_config: AC,
    ) -> Result<&'static Self, Self::Error> {
        info!("Initializing app with config: {:?}", &app_config);
        let persister = AppStateFilePersister::new(&app_config)?;
        // not using SINGLETON.get_or_init() so we can propergate the AppStatePersistError
        let result = match SINGLETON.get() {
            Some(instance) => Ok(instance),
            None => {
                let app_state = match persister.load_app_state::<AppConfigImpl, AppStateImpl>() {
                    Ok(app_state) => app_state,
                    Err(AppStateFilePersisterError::FileNotFound(file_path)) => {
                        // todo match on IO-FileNotFound or avoid this error type duplication
                        // | AppStateFilePersisterError::IOError(io_Error, file_path)
                        //     if io_Error.kind() == IoErrorKind::NotFound
                        info!(
                            "No app state file found in {:?}, creating new state there.",
                            &file_path
                        );
                        let app_state = AppState::new(&app_config);
                        persister.persist_app_state::<AppStateImpl>(&app_state)?;
                        app_state
                    }
                    Err(AppStateFilePersisterError::IOError(io_err, path)) => {
                        return Err(Self::Error::from((io_err, path)));
                    }
                    Err(AppStateFilePersisterError::DeserializationError(err, path)) => {
                        return Err(Self::Error::from((err, path)));
                    }
                };

                let app_config_impl =
                    AppConfigImpl::new(Some(app_config.borrow_app_state_url().to_string()));
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

    // frb doesn't support generics. Thus, we can call this concrete function.
    fn initialise(app_state_url: Option<String>) -> Result<(), AppStateFilePersisterError> {
        Self::initialise_with_app_config(AppConfigImpl::new(app_state_url))?;
        Ok(())
    }

    fn get_singleton() -> &'static Self {
        SINGLETON.get().expect(
            "Lifecycle: should been initialized with Lifecycle::new(AppConfig, AppStatePersister, AppStatePersisterError)!",
        )
    }

    /// persist the app state to the previously stored location
    fn persist() -> Result<(), ProcessingError> {
        let lifecycle = Self::get_singleton();
        lifecycle
            .persister
            .persist_app_state(&lifecycle.app_state)
            .map_err(|err| err.to_processing_error())
    }

    fn shutdown() -> Result<(), ProcessingError> {
        info!("shutting down the app");
        // blocks on the Locks of inner fields
        // TODO implent timeout and throw an error?
        Self::persist()
    }
}
