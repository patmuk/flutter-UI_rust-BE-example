// use std::path::Path;

use app_core::application::api::lifecycle::Lifecycle;
use app_core::application::api::todo_list_api::{self, Effect};

fn main() {
    // initiializes the app and loads the app state
    Lifecycle::new(Some("./test_app_state.bin".to_string()));

    // following CQRS, this is how to query for the state
    let effects = todo_list_api::process_query(todo_list_api::Query::AllTodos);
    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    handle_effects(&effects);

    let effects = todo_list_api::process_command(todo_list_api::Command::AddTodo(
        "Proper Test todo".to_string(),
    ))
    .expect("Could not persist the state!");
    println!("added todo !");
    handle_effects(&effects);

    // if possible call this function to clean up, like wrting the app's state to disk
    Lifecycle::get()
        .shutdown()
        .expect("Could not persist the state!");
}

fn handle_effects(effects: &Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::RenderTodoList(todo_list_model) => {
                println!("Rendering view:\n");
                todo_list_model
                    .blocking_read()
                    .get_todos_as_string()
                    .iter()
                    .for_each(|todo| println!("   - {:?}", todo))
            }
        }
    }
}
