use generate_cqrs_api_macro::generate_api;
use log::trace;

use crate::application::app_state::AppStateImpl;
use std::io;
use std::sync::OnceLock;

static SINGLETON: OnceLock<LifecycleImpl> = OnceLock::new();

pub struct LifecycleImpl {
    // the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    pub(crate) app_config: AppConfigImpl,
    // the app state itself doesn't change, only the fields, which are behind a Mutex to be thread save.
    pub(crate) app_state: AppStateImpl,
}

#[generate_api(
    "app_core/src/domain/todo_list.rs",
    "app_core/src/domain/todo_category.rs"
)]
impl Lifecycle for LifecycleImpl {
    // to avoid an illegal state (app state not loaded) we do the setup and init in one go
    fn new(path: Option<String>) -> &'static Self {
        SINGLETON.get_or_init(|| {
            let app_config = AppConfig::new(path);
            let app_state = Self::init(&app_config);

            LifecycleImpl {
                app_config,
                app_state,
            }
        })
    }

    fn get_singleton() -> &'static Self {
        SINGLETON
            .get()
            .expect("Lifecycle: should been initialized with  ::new()!")
    }

    fn app_config(&self) -> &impl AppConfig {
        &self.app_config
    }

    fn app_state(&self) -> &impl AppState {
        &self.app_state
    }

    /// persist the app state to the previously stored location
    fn persist(&self) -> Result<(), io::Error> {
        self.app_state
            .persist_to_path(&self.app_config.app_state_file_path)
    }

    fn shutdown(&self) -> Result<(), io::Error> {
        debug!("shutting down the app");
        // blocks on the Locks of inner fields
        // TODO implent timeout and throw an error?
        self.app_state
            .persist_to_path(AppConfig::get_app_state_file_path(&self.app_config))
    }
}
// app state storage location
#[derive(Debug)]
pub struct AppConfigImpl {
    pub app_state_file_path: PathBuf,
}

impl AppConfig for AppConfigImpl {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(path: Option<String>) -> Self {
        match path {
            Some(path) => {
                trace!(
                "Overwriting default setup:\n  - setting the app_state_storage_path to {path:?}"
            );
                AppConfigImpl {
                    app_state_file_path: PathBuf::from(path),
                }
            }
            None => {
                debug!("Using default path in setup");
                AppConfigImpl::default()
            }
        }
    }
    fn get_app_state_file_path(&self) -> &PathBuf {
        &self.app_state_file_path
    }
}

impl Default for AppConfigImpl {
    fn default() -> Self {
        AppConfigImpl {
            app_state_file_path: PathBuf::from("./app_state_model.bin"),
        }
    }
}
