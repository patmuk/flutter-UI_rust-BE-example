use app_core::{
    application::api::{
        lifecycle::{self, shutdown, APP_STATE},
        todo_list_api::{process_command, process_query, Command, Query},
    },
    domain::todo_list::{Effect, TodoListModel},
};
use std::{borrow::Borrow, io, num::ParseIntError, process};

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
    lifecycle::init();
    let effects = process_query(Query::GetModel);
    // if let Effect::Render(initial_todo_list) = effects.first().unwrap() {
    //     println!("Loaded Todo List:\n");
    //     print_todo_list(initial_todo_list);
    // }
    let model = lifecycle::get_state();

    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) if user_input.starts_with('h') => {
                println!("{}", USAGE);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('a') => {
                let todo = user_input.split_at(2).1.trim();
                let effects = process_command(Command::AddTodo(todo.to_string()));
                hande_effects(effects);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('v') => {
                let effects = process_query(Query::GetModel);
                hande_effects(effects);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('r') => {
                let index: Result<u32, ParseIntError> = user_input.split_at(2).1.trim().parse();
                match index {
                    Ok(index) => {
                        if index > 0 {
                            println!("\nRemoving todo at index {}\n", index);
                            let effects = process_command(Command::RemoveTodo(index));
                            hande_effects(effects);
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
                let effects = process_command(Command::CleanList);
                hande_effects(effects);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('q') => {
                shutdown();
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

fn hande_effects(effects: Vec<Effect>) {
    effects.iter().for_each(|effect| match effect {
        Effect::Render => {
            print_todo_list(&APP_STATE);
            // let model_rwlock = lifecycle::get_state();
            // let model = model_rwlock.read();
            // print_todo_list(&model);
        }
        Effect::RenderTodoList => {
            print_todo_list_items(lifecycle::get_state().items);
        }
    });
}

fn print_todo_list(todo_list: &TodoListModel) {
    print_todo_list_items(&todo_list.items)
}
fn print_todo_list_items(todo_list_items: &Vec<String>) {
    println!("\nTodo List with {} items:", todo_list_items.len());
    todo_list_items
        .iter()
        .enumerate()
        .for_each(|(index, item)| {
            println!("\t{}. {}", index + 1, item);
        });
    println!("\n");
}
