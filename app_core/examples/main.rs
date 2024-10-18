use app_core::application::api::lifecycle::{Effect, Lifecycle};
use app_core::application::api::processing::Cqrs;

fn main() {
    // initiializes the app and loads the app state
    let lifecycle = Lifecycle::new(Some("./test_app_state.bin".to_string()));

    // following CQRS, this is how to query for the state
    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    process_and_handle_effects(Cqrs::TodoQueryAllTodos);

    process_and_handle_effects(Cqrs::TodoCommandAddTodo("Proper Test todo".to_string()));

    // if possible call this function to clean up, like wrting the app's state to disk
    lifecycle.shutdown().expect("Could not persist the state!");
}

fn process_and_handle_effects(cqrs: Cqrs) {
    let effects = cqrs.process().expect("failed to process command");
    handle_effects(&effects);
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
