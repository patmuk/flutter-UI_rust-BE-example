use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
};
// use flutter_rust_bridge::{frb, support::lazy_static, RustOpaque};
// not needed by FRB, but needed to handle write access on this global variable!
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::RwLock;

use serde::{Deserialize, Serialize};
// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::{LevelFilter, debug, info, error, trace};
use oslog::OsLogger;

use crate::todo_list::{self, TodoListModel};
pub use crate::todo_list::{Effect, Event, ViewModel};

// only pub functions to call
/// instanciate the lazy static app state -
/// call if you want to control when the app state is initialized,
/// which might take time (due to IO when loading the last saved state)
/// otherwise it is called automatically when the lazy reference is accessed the first time
pub fn init() {
    let _ = &*API;
}

/// call to overwrite default values.
/// Doesn't trigger initialization.
// TODO implement after v2 upgrade
// pub fn setup(app_config: AppConfig) -> Result<(), io::Error> {
pub fn setup(app_config: AppConfig) {
    println!("Checking if we can write to {:?}", app_config.app_state_file_path);
    debug!("Checking if we can write to {:?}", app_config.app_state_file_path);
    // OpenOptions::new().create_new(true).open(&app_config.app_state_file_path)?; // Location is writeable
    OpenOptions::new().create_new(true).open(&app_config.app_state_file_path).unwrap(); // Location is writeable
    // TODO propper error handling!
    APP_CONFIG.set(app_config).unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
    // Ok(())
}
// app state storage location
#[derive(Debug)]
pub struct AppConfig {
  pub app_state_file_path: String,
// not supported in v1, TODO implement after upraged to v2!
//   pub app_state_file_path: &'static str,
}

impl Default for AppConfig {
    fn default() -> Self {
        let app_config = AppConfig{
            app_state_file_path: "./app_state_model.bin".to_string(),
        };
        app_config
    }    
}

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub fn process_event(event: Event) -> Vec<Effect> {
    debug!("Processing event: {:?}", event);
    trace!("got thus trace log message?");
    let effects = todo_list::process_mod_event(event, &mut API.write().model);
    debug!("Processed event, got the effects {:?}", effects);
    // TODO too much IO?
    persist_app_state();//.unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
    debug!("persisted state");
    effects
}

pub fn view() -> ViewModel {
    todo_list::view(&API.read().model)
}

/// Stores the app's state in a file.
///
/// # Errors
///
/// This function will return an error if anything goes wrong
// TODO implement after v2 upgrade
// pub fn persist_app_state() -> Result<(), io::Error> {
pub fn persist_app_state() {
    let app_config = APP_CONFIG.get().expect("AppConfig must be set, error in this lib's logic flow!");
    debug!("persisting app state to {:?}", &app_config.app_state_file_path);

    let serialized_app_state: Vec<u8> =
    bincode::serialize(&app_config.app_state_file_path).expect("bincode.serialzation itself should not result in an error, unless the contract with serde is not respected!");
    // Write the serialized data to the file
    let mut file =
        File::create(&app_config.app_state_file_path).unwrap();
    file.write_all(&serialized_app_state).unwrap();
    debug!("Persisted app state to file: {}", &app_config.app_state_file_path);
    // let mut file =
    //     File::create(&app_config.app_state_file_path)?;
    // file.write_all(&serialized_app_state)?;
    // debug!("Persisted app state to file: {}", &app_config.app_state_file_path);
    // Ok(())
}

pub fn shutdown() {
    persist_app_state();//.unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
}

// initializes the app_state only at first call
// The app state is behind a mutex to avoid data conditions, and static, to be globally available to all threads
// lazy_static! {
// static ref IS_LOG_INITIALIZED: RwLock<bool> = RwLock::new(false);
// static ref API: RwLock<AppState> = RwLock::new(AppState::new());
// }
static API: Lazy<RwLock<AppState>> =
    Lazy::new(|| RwLock::new(AppState::new()));

// holds the complete state of the app, as a global static variable
#[derive(Default, Serialize, Deserialize, Debug)]
// #[frb(non_final)]
struct AppState{
    model: TodoListModel,
}

impl AppState {
    fn new() -> Self {
        //configures the logger
        OsLogger::new("com.example.todo_app")
        .level_filter(LevelFilter::Trace)
        .category_level_filter("Settings", LevelFilter::Trace)
        .init()
        .unwrap();
        debug!("initializing the Rust lib");
        APP_CONFIG.get_or_init(|| AppConfig::default());
        // TODO handle error cases
        let app_state = AppState::load().unwrap();
        info!("Initialization finished, log level is {:?}", log::max_level());
        app_state
    }
    // get the last persisted app state, if any exists
    // this function can only be called once, as it will initialize the app state
    // if it does not exist
    fn load() -> Result<AppState, AppStateLoadError> {
        let app_state: AppState;
        let app_config = APP_CONFIG.get().expect("Application flow error: APP_CONFIG should be set by AppState::new() or by the caller, calling setup()");
        // Attempt to read the file
        match std::fs::read(&app_config.app_state_file_path) {
            Ok(buffer) => {
                // If successful, deserialize and display the struct
                app_state = bincode::deserialize(&buffer).map_err(|e| AppStateLoadError::DeSerizationError(e, &app_config.app_state_file_path))?;
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // If the file does not exist, create a default struct
                app_state = AppState::default();
            }
            Err(err) => {
                // Handle other errors
                error!("Error reading file: {}", err);
                eprintln!("Error reading file: {}", err);
                return Err(err).map_err(|e| AppStateLoadError::ReadFile(e, &app_config.app_state_file_path))?;
            }
        }
        Ok(app_state)
    }
}

// handle errors as suggested by https://kazlauskas.me/entries/errors
#[derive(thiserror::Error, Debug)]
pub enum AppStateLoadError {
    #[error("Cannot read file from path: {1}")]
    ReadFile(#[source] io::Error, &'static str),
    #[error("could not understand (=deserialize) the file {1}. Maybe it's content got corrupted?")]
    DeSerizationError(#[source] bincode::Error, &'static str),
}


// from crux? 
// TODO refactor with FBR v2

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
#[derive(Debug)]
pub enum Platform {
    Unknown,
    Android,
    Ios,
    Windows,
    Unix,
    MacIntel,
    MacApple,
    Wasm,
}

// A function definition in Rust. Similar to Dart, the return type must always be named
// and is never inferred.
pub fn platform() -> Platform {
    // This is a macro, a special expression that expands into code. In Rust, all macros
    // end with an exclamation mark and can be invoked with all kinds of brackets (parentheses,
    // brackets and curly braces). However, certain conventions exist, for example the
    // vector macro is almost always invoked as vec![..].
    //
    // The cfg!() macro returns a boolean value based on the current compiler configuration.
    // When attached to expressions (#[cfg(..)] form), they show or hide the expression at compile time.
    // Here, however, they evaluate to runtime values, which may or may not be optimized out
    // by the compiler. A variety of configurations are demonstrated here which cover most of
    // the modern oeprating systems. Try running the Flutter application on different machines
    // and see if it matches your expected OS.
    //
    // Furthermore, in Rust, the last expression in a function is the return value and does
    // not have the trailing semicolon. This entire if-else chain forms a single expression.
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::Ios
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Platform::MacApple
    } else if cfg!(target_os = "macos") {
        Platform::MacIntel
    } else if cfg!(target_family = "wasm") {
        Platform::Wasm
    } else if cfg!(unix) {
        Platform::Unix
    } else {
        Platform::Unknown
    }
}

// The convention for Rust identifiers is the snake_case,
// and they are automatically converted to camelCase on the Dart side.
pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}
