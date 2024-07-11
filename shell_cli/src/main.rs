// use app_core::api::{self, Effect, ViewModel};
use app_core::api::{
    lifecycle::shutdown,
    todo_list_api::{process_event, view, Effect, Event, ViewModel},
};
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
    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) if user_input.starts_with('h') => {
                println!("{}", USAGE);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('a') => {
                let todo = user_input.split_at(2).1.trim();
                // let app = api::A::new();
                let effects = process_event(Event::AddTodo(todo.to_string()));
                hande_effects(effects);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('v') => {
                hande_effects(vec![Effect::Render(view())]);
                user_input.clear();
            }
            Ok(_) if user_input.starts_with('r') => {
                let index: Result<u32, ParseIntError> = user_input.split_at(2).1.trim().parse();
                match index {
                    Ok(index) => {
                        if index > 0 {
                            println!("\nRemoving todo at index {}\n", index);
                            let effects = process_event(Event::RemoveTodo(index));
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
                let effects = process_event(Event::CleanList);
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
        Effect::Render(view_model) => {
            print_todo_list(&view_model);
        }
    });
}

fn print_todo_list(todo_list: &ViewModel) {
    println!("\nTodo List with {} items:", todo_list.count);
    todo_list
        .items
        .iter()
        .enumerate()
        .for_each(|(index, item)| {
            println!("\t{}. {}", index + 1, item);
        });
    println!("\n");
}
