use app_core::application::api::{
    lifecycle::Lifecycle,
    processing::{process_todo_model_command, process_todo_model_query, TodoCommand, TodoQuery},
};
use app_core::utils::cqrs_traits::Effect;
use std::{io, num::ParseIntError, process};

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    const USAGE: &str = "=====================\n\
                          Sherry CLI\n\
                          =====================\n\
                          type\n\
                          \n\
                          h  ................. to print the test message\n\
                          a <some todo> ...... to add a todo\n\
                          v  ................. to view all todos\n\
                          r <index number>  .. to remove the todo at the given number\n\
                          c  ................. to remove ALL todos\n\
                          q  ................. to quit\n\
                          =====================\n";
    println!("{}", USAGE);
    Lifecycle::new(None);
    let effects = process_query(TodoQuery::AllTodos);
    handle_effects(&effects);

    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) if user_input.starts_with('h') => {
                println!("{}", USAGE);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('a') => {
                let todo = user_input.split_at(2).1.trim();

                process_and_handle_effects(TodoCommand::AddTodo(todo.to_string()));
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('v') => {
                let effects = process_query(TodoQuery::AllTodos);
                handle_effects(&effects);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('r') => {
                let index: Result<u32, ParseIntError> = user_input.split_at(2).1.trim().parse();
                match index {
                    Ok(index) => {
                        if index > 0 {
                            println!("\nRemoving todo at index {}\n", index);
                            let effects =
                                process_and_handle_effects(TodoCommand::RemoveTodo(index))
                                    .expect("failed to remove a todo");
                            handle_effects(&effects);
                        } else {
                            println!("\nGive me a positive number, not {}\n", index);
                        }
                    }
                    Err(err) => {
                        println!("I don't understand the number {:?} you gave me.", err)
                    }
                }
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('c') => {
                let effects = process_and_handle_effects(TodoCommand::CleanList)
                    .expect("failed to clean the list");
                handle_effects(&effects);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('q') => {
                Lifecycle::get().shutdown();
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

fn process_and_handle_effects(cqrs: impl Cqrs) {
    let effects = process_todo_model_command(cqrs).expect("failed to process command");
    handle_effects(effects);
}

fn handle_effects(effects: Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::RenderTodoList(todo_list_model) => {
                println!("Rendering view:\n");
                todo_list_model
                    .blocking_read()
                    .get_todos_as_string()
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| println!("\t{}. {}", index + 1, item))
            }
        }
    }
}
