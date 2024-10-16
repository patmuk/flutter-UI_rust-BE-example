use app_core::application::api::lifecycle::Lifecycle;
use app_core::application::api::processing::{self, process_todo_model_command, Effect};
use app_core::utils::cqrs_traits::Cqrs;

fn main() {
    // initiializes the app and loads the app state
    Lifecycle::new(Some("./test_app_state.bin".to_string()));

    // following CQRS, this is how to query for the state
    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    process_and_handle_effects(processing::Query::AllTodos);

    process_and_handle_effects(processing::Command::AddTodo("Proper Test todo".to_string()));

    // if possible call this function to clean up, like wrting the app's state to disk
    Lifecycle::get()
        .shutdown()
        .expect("Could not persist the state!");
}

fn process_and_handle_effects(cqrs: impl Cqrs) {
    let effects = process_todo_model_command(cqrs)
        // .process()
        .expect("failed to process command");
    handle_effects(&effects);
}
fn handle_effects(effects: &Vec<impl Effect>) {
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
