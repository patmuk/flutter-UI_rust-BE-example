use log::{debug, trace};

use crate::application::app_state::AppState;
pub use crate::application::processing_errors::ProcessingError;
pub use crate::domain::effects::Effect;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

static SINGLETON: OnceLock<Lifecycle> = OnceLock::new();

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
        self.app_state
            .persist_to_path(&self.app_config.app_state_file_path)
    }
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
