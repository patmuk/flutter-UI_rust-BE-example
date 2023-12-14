use std::{fs::File, io::{Write, self}};

use anyhow::Context;
use flutter_rust_bridge::{support::lazy_static, frb};
use serde::{Serialize, Deserialize};
use parking_lot::RwLock;
// probably not needed, rust borrowing rules should be enought and concurrency should be handeled by flutter-rust-bridge
// use std::sync::RwLock;

pub use crate::todo_list::{Effect, Event, ViewModel};
use crate::todo_list::{TodoListModel, self};

// holds the complete state of the app, as a global static variable
#[derive(Default, Serialize, Deserialize, Debug)]
#[frb(non_final)]
struct AppState {
    // pub model: Box<TodoListModel>,
    pub model: TodoListModel,
}
// initializes the app_state only at first call
// The app state is behind a mutex to avoid data conditions, and static, to be globally available to all threads
lazy_static! {
    // TODO handle, if state cannot be loaded!
    static ref APP_STATE: RwLock<AppState> = RwLock::new(load_app_state().unwrap());
}

// handle app_state persistence
const APP_STATE_FILE_PATH: &str = "app_state_model.bin";
// get the last persisted app state, if any exists
    fn load_app_state() -> anyhow::Result<AppState> {
        let app_state: AppState;
    // Attempt to read the file
     match std::fs::read(APP_STATE_FILE_PATH) {
    Ok(buffer) => {
        // If successful, deserialize and display the struct
        let model: TodoListModel =  bincode::deserialize(&buffer)?;
        app_state = AppState { model };
        // println!("{:?}", app_state);
    }
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
        // If the file does not exist, create a default struct
        app_state = AppState::default();
        // println!("File does not exist. Using default struct: {:?}", app_state);
    }
    Err(err) => {
        // Handle other errors
        // eprintln!("Error reading file: {}", err);
        return Err(anyhow::Error::new(err));
    }
    }
    Ok(app_state)
}

/// Stores the app's state in a file.
///
/// # Errors
///
/// This function will return an error if anything goes wrong
fn persist_app_state() -> anyhow::Result<()> {
    let serialized_model: Vec<u8> = bincode::serialize(&(APP_STATE.read().model))
        .context("Error serializing the model")?;

    // Write the serialized data to the file
    let mut file = File::create(APP_STATE_FILE_PATH)
        .context("Error creating the app state file")?;
    file.write_all(&serialized_model)
        .context("Error writing to the app state file")?;

    Ok(())    
    // let serialized_model = serde_json::to_string(&app_state.model).unwrap();
    // let mut file = File::create("app_state.json").unwrap();
    // file.write_all(serialized_model.as_bytes()).unwrap();
}

pub fn process_event(event: Event) -> Vec<Effect> {
    let result = todo_list::process_mod_event(event, &mut APP_STATE.write().model);
    // TODO persist less ofter
    persist_app_state().unwrap();
    result
}

pub fn view() -> ViewModel {    
    todo_list::view(&APP_STATE.read().model)
}



// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

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
