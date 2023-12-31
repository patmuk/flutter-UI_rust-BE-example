use crate::{
    app_state::{self, AppState},
    todo_list, ensure_logger_is_set_up,
};

use std::path::PathBuf;
// use flutter_rust_bridge::{frb, support::lazy_static, RustOpaque};
// not needed by FRB, but needed to handle write access on this global variable!
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::RwLock;

// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::{debug, error, trace};

pub use crate::todo_list::{Effect, Event, ViewModel};

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

pub fn process_event(event: Event) -> Vec<Effect> {
    debug!("Processing event: {:?}", event);
    let effects = todo_list::process_mod_event(event, &mut API.write().model);
    debug!("Processed event, got the effects {:?}", effects);
    // TODO too much IO?
    persist_app_state(); //.unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
    effects
}

pub fn view() -> ViewModel {
    todo_list::view(&API.read().model)
}

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
static API: Lazy<RwLock<AppState>> = Lazy::new(|| {
    RwLock::new(AppState::new(
        APP_CONFIG.get_or_init(|| AppConfig::default()),
    ))
});

// from crux?
// TODO refactor with FBR v2

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
// #[derive(Debug)]
// pub enum Platform {
//     Unknown,
//     Android,
//     Ios,
//     Windows,
//     Unix,
//     MacIntel,
//     MacApple,
//     Wasm,
// }

// // A function definition in Rust. Similar to Dart, the return type must always be named
// // and is never inferred.
// pub fn platform() -> Platform {
//     // This is a macro, a special expression that expands into code. In Rust, all macros
//     // end with an exclamation mark and can be invoked with all kinds of brackets (parentheses,
//     // brackets and curly braces). However, certain conventions exist, for example the
//     // vector macro is almost always invoked as vec![..].
//     //
//     // The cfg!() macro returns a boolean value based on the current compiler configuration.
//     // When attached to expressions (#[cfg(..)] form), they show or hide the expression at compile time.
//     // Here, however, they evaluate to runtime values, which may or may not be optimized out
//     // by the compiler. A variety of configurations are demonstrated here which cover most of
//     // the modern oeprating systems. Try running the Flutter application on different machines
//     // and see if it matches your expected OS.
//     //
//     // Furthermore, in Rust, the last expression in a function is the return value and does
//     // not have the trailing semicolon. This entire if-else chain forms a single expression.
//     if cfg!(windows) {
//         Platform::Windows
//     } else if cfg!(target_os = "android") {
//         Platform::Android
//     } else if cfg!(target_os = "ios") {
//         Platform::Ios
//     } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
//         Platform::MacApple
//     } else if cfg!(target_os = "macos") {
//         Platform::MacIntel
//     } else if cfg!(target_family = "wasm") {
//         Platform::Wasm
//     } else if cfg!(unix) {
//         Platform::Unix
//     } else {
//         Platform::Unknown
//     }
// }
