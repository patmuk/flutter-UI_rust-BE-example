use bcs::{from_bytes, to_bytes};
use core::{handle_response, process_event, Effect, Event, ViewModel, Request};
use std::io;

fn main() {
    //    let args = Args::parse();
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut response: Vec<u8>;
    let mut view_model = from_bytes::<ViewModel>(&core::view()).unwrap();
    loop {
        println!("Todo List \n 'd + index' to delete an entry, enter en entry to add.");
        println!("----- {} TODOs -----", view_model.count);
        println!("{:#?}", view_model.items);
        match stdin.read_line(&mut user_input) {
            Ok(_) if user_input.starts_with('d') => {
                let index: usize = user_input[1..user_input.len() - 1]
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("not a number: '{}'", &user_input[1..]));
                println!("deleting todo at line {}", index);
                response = process_event(&to_bytes(&Event::RemoveTodo(index)).unwrap());
                // println!("response: '{:?}'", from_bytes::<Effect>(&response));
            }
            Ok(_) => {
                let todo = &user_input;
                response = process_event(
                    &to_bytes(&Event::AddTodo(todo.into())).unwrap(),
                );
                // println!("response: '{:?}'", from_bytes::<Request<Effect>>(&response));
            }
            Err(_) => panic!("input not comprehensible {}", user_input),
        };
        user_input.clear();

        let reqs: Vec<Request<Effect>> = from_bytes(&response).unwrap();
        for Request { uuid, effect } in reqs {
            match effect {
                Effect::Render(_) => {
                    view_model = from_bytes::<ViewModel>(&core::view()).unwrap();
                    println!(
                        "Todo List with {} items:\n{:#?}",
                        view_model.count, view_model.items
                    )
                }
            }
        }
    }
}
