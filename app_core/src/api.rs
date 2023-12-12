use flutter_rust_bridge::{support::lazy_static, frb};
use parking_lot::RwLock;
// probably not needed, rust borrowing rules should be enought and concurrency should be handeled by flutter-rust-bridge
// use std::sync::RwLock;

pub use crate::todo_list::{Effect, Event, ViewModel};
use crate::todo_list::{TodoListModel, self};

// holds the complete state of the app, as a global static variable
#[derive(Default)]
#[frb(non_final)]
struct AppState {
    // pub model: Box<TodoListModel>,
    pub model: TodoListModel,
}
// initializes the app_state only at first call
// The app state is behind a mutex to avoid race conditions

lazy_static! {
    static ref APP_STATE: RwLock<AppState> = RwLock::new(AppState::default());
}
            

pub fn process_event(event: Event) -> Vec<Effect> {
    todo_list::process_mod_event(event, &mut APP_STATE.write().model)
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
