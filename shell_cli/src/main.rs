use core::lib_crux;
// use bcs::{from_bytes, to_bytes};
// use core::{handle_response, process_event, Effect, Event, ViewModel, Request};
use log::debug;
use std::io;

fn main() {
    simple_logger::init().unwrap();
    //    let args = Args::parse();
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut effects;
    let mut view_model = core::lib_crux::view();
    println!("=====================");
    println!("Todo List CLI");
    println!("enter an entry to add or\nthe number of the line to delete");
    println!("=====================");
    loop {
        println!("\n\n\n----- {} TODOs -----", view_model.count);
        for (index, item) in view_model.items.iter().enumerate() {
            println!("{}....{item}",index+1);
        }
        stdin
            .read_line(&mut user_input)
            .expect("Could not read user input!");

        if let Ok(index) = user_input.trim().parse::<usize>() {
            println!("deleting todo at line {}", index);
            effects = lib_crux::process_event(lib_crux::Event::RemoveTodo(index));
        } else {
            let todo = user_input.trim();
            effects = lib_crux::process_event(lib_crux::Event::AddTodo(todo.to_string()));
        }

        user_input.clear();

        for effect in effects {
            match effect {
                lib_crux::Effect::Render(_) => {
                    view_model = lib_crux::view();
                    for (index, item) in view_model.items.iter().enumerate() {
                        println!("{}....{}", index + 1, item);
                    }
                }
            }
        }
    }
}
