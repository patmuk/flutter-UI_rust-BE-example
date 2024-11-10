use app_core::application::api::api_traits::Lifecycle;
use app_core::application::api::lifecycle::LifecycleImpl;
use app_core::application::api::processing::{Effect, ProcessingError, TodoCommand, TodoQuery};
use app_core::utils::cqrs_traits::Cqrs;
use std::error::Error;
use std::process;

fn main() {
    let mut user_input = String::new();
    let stdin = std::io::stdin();

    const USAGE: &str = "=====================\n\
                          Sherry CLI\n\
                          =====================\n\
                          type\n\
                          \n\
                          h  ................. to print the test message\n\
                          a <some todo> ...... to add a todo\n\
                          v  ................. to view all todos\n\
                          n <index number>  .. to view todo number <index>\n\
                          r <index number>  .. to remove the todo at the given number\n\
                          c  ................. to remove ALL todos\n\
                          q  ................. to quit\n\
                          =====================\n";
    println!("{}", USAGE);
    let lifecycle: &LifecycleImpl = Lifecycle::new(None);
    let _ = process_and_handle_effects(TodoQuery::AllTodos);

    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) if user_input.starts_with('h') => {
                println!("{}", USAGE);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('a') => {
                let todo = user_input.split_at(2).1.trim();

                let _ = process_and_handle_effects(TodoCommand::AddTodo(todo.to_string()))
                    .expect("Could not add a todo");
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('v') => {
                let _ = process_and_handle_effects(TodoQuery::AllTodos);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('n') => {
                let index = parse_index(&user_input).expect("Your command is wrong: {}");
                println!("\nShowing (only) todo at index {}\n", index);
                let _ = process_and_handle_effects(TodoQuery::GetTodo(index))
                    .expect("failed to remove a todo");
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('r') => {
                let index = parse_index(&user_input).expect("Wrong command: ");
                println!("\nRemoving todo at index {}\n", index);
                let _ = process_and_handle_effects(TodoCommand::RemoveTodo(index))
                    .expect("failed to remove a todo");
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('c') => {
                process_and_handle_effects(TodoCommand::CleanList)
                    .expect("failed to clean the list");
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('q') => {
                let _ = lifecycle.shutdown();
                process::exit(0);
            }
            Ok(_) => {
                println!("\nI don't understand your command '{user_input}'");
                user_input.clear()
            }
            Err(_) => panic!("\ninput not comprehensible {user_input}"),
        };
    }
}

fn parse_index(user_input: &String) -> Result<usize, Box<dyn Error>> {
    match user_input.split_at(2).1.trim().parse() {
        Ok(index) => {
            if index > 0 {
                Ok(index)
            } else {
                Err(format!("\nGive me a positive number, not {}\n", index).into())
            }
        }
        Err(err) => Err(format!(
            "I don't understand the number {:?} you gave me.\n{}",
            user_input, err
        )
        .into()),
    }
}

fn process_and_handle_effects(cqrs: impl Cqrs) -> Result<(), ProcessingError> {
    let effects = cqrs.process()?;
    Ok(handle_effects(effects))
}

fn handle_effects(effects: Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::TodoListEffectRenderTodoList(todo_list_model_lock) => {
                println!("Rendering view:\n");
                todo_list_model_lock
                    .lock
                    .blocking_read()
                    .get_todos_as_string()
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| println!("\t{}. {}", index + 1, item))
            }
            Effect::TodoListEffectRenderTodoItem(todo_item) => {
                println!("\tGot todo item: {}", todo_item.text)
            }
        }
    }
}
