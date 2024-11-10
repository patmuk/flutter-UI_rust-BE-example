use app_core::application::api::api_traits::Lifecycle;
use app_core::application::api::lifecycle::LifecycleImpl;
use app_core::application::api::processing::{Effect, ProcessingError, TodoCommand, TodoQuery};
use app_core::utils::cqrs_traits::Cqrs;

fn main() {
    // initiializes the app and loads the app state
    let lifecycle: &LifecycleImpl = Lifecycle::new(Some("./test_app_state.bin".to_string()));

    // following CQRS, this is how to query for the state
    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    process_and_handle_effects(TodoQuery::AllTodos).expect("Could not read all todo's!");

    process_and_handle_effects(TodoCommand::AddTodo("Proper Test todo".to_string()))
        .expect("Couldn't add a todo!");

    // if possible call this function to clean up, like wrting the app's state to disk
    lifecycle.shutdown().expect("Could not persist the state!");
}

fn process_and_handle_effects(cqrs: impl Cqrs) -> Result<(), ProcessingError> {
    let effects = cqrs.process()?;
    Ok(handle_effects(&effects))
}
fn handle_effects(effects: &Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::TodoListEffectRenderTodoList(todo_list_model_lock) => {
                println!("Rendering view:\n");
                todo_list_model_lock
                    .lock
                    .blocking_read()
                    .get_todos_as_string()
                    .iter()
                    .for_each(|todo| println!("   - {:?}", todo))
            }
            Effect::TodoListEffectRenderTodoItem(todo_item) => {
                println!("got item {}", todo_item.text);
            }
        }
    }
}
