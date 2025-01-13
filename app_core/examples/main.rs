use app_core::application::app_config::AppConfigImpl;
use app_core::{
    application::api::lifecycle::*,
    // domain::{todo_category, todo_list},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // condigure the app
    let app_config: AppConfigImpl = AppConfig::new(Some("./test_app_state.bin".to_string()));
    // this loads or created the state

    // instead of the full annotated new() call the factory
    use app_core::infrastructure::app_state_file_persister::AppStateFilePersister;
    use app_core::infrastructure::app_state_file_persister::AppStateFilePersisterError;
    let lifecycle: &LifecycleImpl =
        Lifecycle::new::<AppConfigImpl, AppStateFilePersister, AppStateFilePersisterError>(
            app_config,
        )
        .expect("App state should have loaded.");
    // LifecycleImpl::new_with_file_persister(app_config).expect("App state should have loaded.");

    // following CQRS, this is how to query for the state
    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    // process_and_handle_effects cares about reacting to the returned effects. It is recommended to do this in a
    // central location like this function
    process_and_handle_effects(TodoCategoryModelQuery::GetTodoCategoryModel)
        .expect("Could not read all todo's!");

    process_and_handle_effects(TodoCategoryModelCommand::UpdateTitle(
        "Test Todo List".to_string(),
    ))?;
    process_and_handle_effects(TodoListModelCommand::AddTodo("Test todo entry".to_string()))
        .expect("Couldn't add a todo!");

    // this is how one calls a CQRS-function and handles the returned effects directly
    let effect = TodoListModelQuery::GetAllTodos
        .process()
        .expect("Couldn't get todo list handle!");

    let todo_list_handle = match &effect[0] {
        Effect::TodoListModelRenderTodoList(todo_list_model_lock) => todo_list_model_lock,
        _ => panic!("Unexpected effect when getting the todo list's handle!"),
    };

    process_and_handle_effects(TodoCategoryModelCommand::SetTodoList(
        todo_list_handle.clone(),
    ))?;

    // if possible call this function to clean up, like wrting the app's state to disk
    Ok(lifecycle.shutdown()?)
}

fn process_and_handle_effects(cqrs: impl Cqrs) -> Result<(), ProcessingError> {
    let effects = cqrs.process()?;
    Ok(handle_effects(&effects))
}
fn handle_effects(effects: &Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::TodoCategoryModelRenderTodoList(todo_list_model_lock)
            | Effect::TodoListModelRenderTodoList(todo_list_model_lock) => {
                println!("Rendering view:\n");
                print_todo_list(
                    "Todo list without title".to_string(),
                    todo_list_model_lock
                        .lock
                        .blocking_read()
                        .get_todos_as_string(),
                );
            }
            Effect::TodoListModelRenderTodoItem(todo_item) => {
                println!("got item {}", todo_item.text);
            }
            Effect::TodoCategoryModelRenderTodoCategoryModel(todo_category_model_lock) => {
                println!("Rendering view with category:\n");
                print_todo_list(
                    todo_category_model_lock.lock.blocking_read().get_title(),
                    todo_category_model_lock.lock.blocking_read().get_todos(),
                );
            }
            Effect::TodoCategoryModelUpdateTitel(todo_category_title)
            | Effect::TodoCategoryModelRenderTodoCategory(todo_category_title) => {
                println!("Category title: {todo_category_title}\n");
            }
        }
    }
    fn print_todo_list(category_title: String, todo_list: Vec<String>) {
        println!("======= {} ========", category_title);
        todo_list
            .iter()
            .for_each(|todo| println!("   - {:?}", todo));
    }
}
