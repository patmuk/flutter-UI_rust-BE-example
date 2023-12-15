use std::{
    fs::File,
    io::{self, Write},
};
use anyhow::Context;
use flutter_rust_bridge::{frb, support::lazy_static, RustOpaque};
// not needed by FRB, but needed to handle write access on this global variable!
use once_cell::sync::Lazy;
use parking_lot::RwLock;

use serde::{Deserialize, Serialize};
// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::{debug, info, LevelFilter};
use oslog::OsLogger;

use crate::todo_list::{self, TodoListModel};
pub use crate::todo_list::{Effect, Event, ViewModel};

// only pub functions to call
/// instanciate the lazy static app state -
/// call if you wnat to controll when the app state is initialized,
/// which might take time (due to IO when loading the last saved state)
pub fn init() {
    // let _ = parking_lot::lock_api::RwLockWriteGuard::<'_, parking_lot::RawRwLock, AppState>::map(API.write(), |mut guard| *guard = AppState::new());
    let _ = &*API;
}

pub fn process_event(event: Event) -> Vec<Effect> {    
    let result = todo_list::process_mod_event(event, &mut API.write().model);
    // TODO too much IO?
    persist_app_state().expect("Error persisting app state");
    result
}

pub fn view() -> ViewModel {
    todo_list::view(&API.read().model)
}

/// Stores the app's state in a file.
///
/// # Errors
///
/// This function will return an error if anything goes wrong
pub fn persist_app_state() -> anyhow::Result<()> {
    let serialized_app_state: Vec<u8> =
        bincode::serialize(&*API.read()).context("Error serializing the model")?;
    // Write the serialized data to the file
    let mut file =
        File::create(APP_STATE_FILE_PATH).context("Error creating the app state file")?;
    file.write_all(&serialized_app_state)
        .context("Error writing to the app state file")?;
    debug!(
        "debug: Persisted app state to file: {}",
        APP_STATE_FILE_PATH
    );
    info!("info: Persisted app state to file: {}", APP_STATE_FILE_PATH);
    Ok(())
}

pub fn shutdown() {
    persist_app_state().expect("Error persisting app state");
}

// handle app_state persistence
const APP_STATE_FILE_PATH: &str = "./app_state_model.bin";

// initializes the app_state only at first call
// The app state is behind a mutex to avoid data conditions, and static, to be globally available to all threads
// lazy_static! {
// static ref IS_LOG_INITIALIZED: RwLock<bool> = RwLock::new(false);
// static ref API: RwLock<AppState> = RwLock::new(AppState::new());
// }
static API: Lazy<RwLock<AppState>> =
    Lazy::new(|| RwLock::new(AppState::new()));

//Mutable statics are safe to access
// #[dynamic(lazy, finalize)]
// static mut AppState: AppState = AppState::new();

// holds the complete state of the app, as a global static variable
#[derive(Default, Serialize, Deserialize, Debug)]
// #[frb(non_final)]
struct AppState{
    model: TodoListModel,
}

impl AppState {
    fn new() -> Self {
        //configures the logger
        OsLogger::new("com.example")
            .level_filter(LevelFilter::Debug)
            .category_level_filter("Settings", LevelFilter::Trace)
            .init()
            .unwrap();
        // TODO handle error cases
        AppState::load().unwrap()
    }
    // get the last persisted app state, if any exists
    // this function can only be called once, as it will initialize the app state
    // if it does not exist
    fn load() -> anyhow::Result<AppState> {
        let app_state: AppState;
        // Attempt to read the file
        match std::fs::read(APP_STATE_FILE_PATH) {
            Ok(buffer) => {
                // If successful, deserialize and display the struct
                app_state = bincode::deserialize(&buffer)?;
                // println!("{:?}", app_state);
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // If the file does not exist, create a default struct
                app_state = AppState::default();
                // println!("File does not exist. Using default struct: {:?}", app_state);
            }
            Err(err) => {
                // Handle other errors
                eprintln!("Error reading file: {}", err);
                // panic!("Error reading file: {}", err);
                return Err(anyhow::Error::new(err));
            }
        }
        Ok(app_state)
    }
}

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
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
