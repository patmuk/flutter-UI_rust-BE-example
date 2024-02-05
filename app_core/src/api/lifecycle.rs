use crate::ensure_logger_is_set_up;

use crate::app_state::{self, AppState};

use std::path::PathBuf;
// use flutter_rust_bridge::{frb, support::lazy_static, RustOpaque};
// not needed by FRB, but needed to handle write access on this global variable!
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::RwLock;

// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::{debug, error, trace};

// pub use crate::todo_list::{Effect, Event, ViewModel};

// only pub functions to call
/// instanciate the lazy static app state -
/// call if you want to control when the app state is initialized,
/// which might take time (due to IO when loading the last saved state)
/// otherwise it is called automatically when the lazy reference is accessed the first time
pub fn init() {
    ensure_logger_is_set_up();
    let _ = &*API;
}

/// call to overwrite default values.
/// Doesn't trigger initialization.
// TODO implement after v2 upgrade
// pub fn setup(app_config: AppConfig) -> Result<(), io::Error> {
// pub fn setup(app_config: AppConfig) {
pub fn setup(path: String) {
    ensure_logger_is_set_up();
    debug!("Overwriting default setup:\n  - setting the app_state_storage_path to {path:?}");
    trace!("Overwriting default setup:\n  - setting the app_state_storage_path to {path:?}");
    let app_config = AppConfig {
        app_state_file_path: PathBuf::from(path),
    };

    // TODO propper error handling!
    APP_CONFIG
        .set(app_config)
        .unwrap_or_else(|err| error!("Error setting the App Configuration: {:?}", err));
}
// app state storage location
#[derive(Debug)]
pub struct AppConfig {
    pub app_state_file_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        let app_config = AppConfig {
            app_state_file_path: PathBuf::from("./app_state_model.bin"),
        };
        app_config
    }
}

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

// pub fn persist_app_state() -> Result<(), std::io::Error> {
pub fn persist_app_state() {
    let app_config = APP_CONFIG
        .get()
        .expect("AppConfig must be set, error in this lib's logic flow!");
    app_state::persist_app_state(&*API.read(), &app_config.app_state_file_path).unwrap();
}

// pub fn shutdown() -> Result<(), std::io::Error> {
pub fn shutdown() {
    debug!("shutting down the app");
    persist_app_state()
}

// initializes the app_state only at first call
// The app state is behind a mutex to avoid data conditions, and static, to be globally available to all threads
pub(crate) static API: Lazy<RwLock<AppState>> = Lazy::new(|| {
    RwLock::new(AppState::new(
        APP_CONFIG.get_or_init(|| AppConfig::default()),
    ))
});
