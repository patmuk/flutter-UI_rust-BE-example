// use std::path::Path;

use app_core::application::api::lifecycle;
use app_core::application::api::todo_list_api;
use app_core::application::api::todo_list_api::Effect;

// initializes and logs a test message using OS logger
// use app_core::api::lifecycle;

fn main() {
    // Send a test log message
    log::info!("Executing the example code.");

    // app.init() is automatically called on first access to api.
    // However, you can call it manually if you want to trigger the initialization.
    // As this attempts to load the app state from disk, it could take a while for IO.

    // optionally set up the file path to store the app state
    lifecycle::setup("./test_app_state.bin".to_string());

    // following CQRS, this is how to query for the state
    let effects = todo_list_api::process_query(todo_list_api::Query::GetModel);
    println!("Loaded Todos: {:?}", effects.first().unwrap());

    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    let effects = todo_list_api::process_command(todo_list_api::Command::AddTodo(
        "Proper Test todo".to_string(),
    ));
    handle_effects(&effects);

    // if possible call this function to clean up, like wrting the app's state to disk
    lifecycle::shutdown();
}

fn handle_effects(effects: &Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::Render(view) => {
                println!("Rendering view: {:?}", view);
            }
            Effect::RenderTodoList(items) => {
                println!("Rendering view: {:?}", items);
            }
        }
    }
}
