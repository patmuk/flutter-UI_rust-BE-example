use crate::domain::app_state::{self, AppState};
use crate::ensure_logger_is_set_up;

use flutter_rust_bridge::frb;
use std::io;
// use parking_lot::lock_api::RwLock;
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};
// use flutter_rust_bridge::{frb, support::lazy_static, RustOpaque};
// not needed by FRB, but needed to handle write access on this global variable!
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
    // let _ = &*API;
    // let _ = &*APP_STATE;
    get_state();
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
        AppConfig {
            app_state_file_path: PathBuf::from("./app_state_model.bin"),
        }
    }
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

// TODO implement Error handling!
// pub fn persist_app_state() -> Result<(), std::io::Error> {
pub fn persist_app_state(app_state: &AppState) {
    let app_config = APP_CONFIG
        .get()
        .expect("AppConfig must be set, error in this lib's logic flow!");
    app_state.persist(&app_config.app_state_file_path).unwrap();
}

// pub fn shutdown() -> Result<(), std::io::Error> {
/// Blocks, if RwLock<AppState> is in write use
/// TODO implent timeout and throw an error?
pub fn shutdown() -> Result<(), io::Error> {
    debug!("shutting down the app");
    match APP_STATE.get() {
        None => Ok(()), //if the state has not been initialized, no need to persist!
        Some(app_state_lock) => {
            app_state_lock.read().persist(
                &APP_CONFIG
                    .get_or_init(AppConfig::default)
                    .app_state_file_path,
            );
            Ok(())
        }
    }
}

// initializes the app_state only at first call
// The app state is behind a mutex to avoid data conditions, and static, to be globally available to all threads
// This is needed, as a static mut cannot be modified by save code, the mutex makes this possible
pub(crate) static APP_STATE: LazyLock<RwLock<AppState>> = LazyLock::new(|| {
    RwLock::new(AppState::load_or_new(
        APP_CONFIG.get_or_init(AppConfig::default),
    ))
});
pub fn get_state() -> &'static RwLock<AppState> {
    APP_STATE.get_or_init(init_app_state)
}
// pub fn get_state_mut() -> &'static mut AppState {
//     match (APP_STATE.get_mut()) {
//         Some(app_state) => {
//             return app_state;
//         }
//         None => {
//             APP_STATE
//                 .set(init_app_state())
//                 .expect("APP_STATE has not been set before,");
//             get_state_mut()
//         }
//     }
//     // APP_STATE.get_or_init(|| {
//     //     let mut app_state = init_app_state();
//     //     persist_app_state()
//     //    .expect("Error persisting the app state");
//     // }
// }
//&RwLockReadGuard<AppState> {
//     match APP_STATE.get() {
//         Some(app_state) => {
//             debug!("App State already exists, returning it");
//             &app_state
//         }
//         None => {
//             init_app_state();
//             get_state()
//         }
//     }
// }
fn init_app_state() -> RwLock<AppState> {
    RwLock::new(AppState::load_or_new(
        APP_CONFIG.get_or_init(AppConfig::default),
    ))
    // APP_STATE
    //     .set(app_state)
    //     .expect("APP_STATE hasn't been set before");
}
// pub fn get_state() -> &'static RwLock<AppState> {
//     //&RwLockReadGuard<AppState> {
//     match APP_STATE.get() {
//         Some(app_state) => {
//             debug!("App State already exists, returning it");
//             app_state
//         }
//         None => {
//             APP_STATE
//                 .set(RwLock::new(AppState::load_or_new(
//                     APP_CONFIG.get_or_init(AppConfig::default),
//                 )))
//                 .expect("APP_STATE hasn't been set before");
//             &APP_STATE.get().expect("ÄPP_STATE just set")
//         }
//     }
// }

// pub static APP_STATE: Lazy<RwLock<AppState>> = Lazy::new(|| {
//     RwLock::new(AppState::load_or_new(
//         APP_CONFIG.get_or_init(AppConfig::default),
//     ))
// });

static APP_STATE: OnceCell<RwLock<AppState>> = OnceCell::new();
// static APP_STATE: OnceCell<RwLock<AppState>> = OnceCell::new();
