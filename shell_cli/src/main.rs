use app_core::api;
use std::io;

fn main() {
    //    let args = Args::parse();
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        println!("=====================");
        println!("Sherry CLI");
        println!("t to print the test message");
        println!("=====================");
        match stdin.read_line(&mut user_input) {
            Ok(_) if user_input.starts_with('t') => {
                println!("Model is {:?}", api::view());
                user_input.clear();
            }
            Ok(_) => {
                println!("\nI don't understand your command '{user_input}'");
                user_input.clear()
            }
            Err(_) => panic!("\ninput not comprehensible {user_input}"),
        };

        // let reqs: Vec<Request<Effect>> = from_bytes(&response).unwrap();
        // for Request { uuid, effect } in reqs {
        //     match effect {
        //         Effect::Render(_) => {
        //             view_model = from_bytes::<TodoListViewModel>(&view()).unwrap();
        //             println!(
        //                 "Todo List with {} items:\n{:#?}",
        //                 view_model.count, view_model.items
        //             )
        //         }
        //     }
        // }
    }
}
