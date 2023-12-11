// use app_core::todo_list::{self, TodoListModel};
use bridge::respond_to_dart;
use messages::todo_list::ViewModel;
use tokio_with_wasm::tokio;

use crate::messages::crux;

pub mod bridge;
pub mod messages;
// mod app_core;

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // initialize the rust side

    // This is `tokio::sync::mpsc::Reciver` that receives the requests from Dart.
    let mut request_receiver = bridge::get_request_receiver();
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    // tokio::spawn(sample_functions::stream_mandelbrot());
    // tokio::spawn(sample_functions::run_debug_tests());
    // while let Some(request_unique) = request_receiver.recv().await {
    //     tokio::spawn(async {
    //         let response_unique = handle_request(request_unique).await;
    //         respond_to_dart(response_unique);
    //     });
    // }

    let mut todo_list_model = TodoListModel { items: Vec::new() };

    // TODO have one Event and Effect declaration, instead of converting between
    pub fn process_event(event: crux::event::Event) -> Vec<crux::effect::Effect> {
        let effect = match event {
            crux::event::Event::AddTodo(add_todo_event) => {
                || {
                    todo_list::update(
                        todo_list::Event::AddTodo { 0: add_todo_event.todo },
                        &mut todo_list_model,
                    )
                }
            },
            crux::event::Event::RemoveTodo(remove_todo_event) => {
                || {
                    todo_list::update(
                        todo_list::Event::RemoveTodo { 0: remove_todo_event.index },
                        &mut todo_list_model,
                    )
                }
            },
            crux::event::Event::CleanList(_) => || {
                todo_list::update(todo_list::Event::CleanList, &mut todo_list_model)
            },
        };
        
        // Execute the closure to get the actual effect
        let effect = effect();
        match effect {
            Some(effect) => vec![effect],
            None => vec![],
        }

        pub async fn handle_response(rust_request: RustRequest) -> RustResponse {
            match rust_request.operation {
                RustOperation::Create => RustResponse::default(),
                RustOperation::Read => {
                    let message_bytes = rust_request.message.unwrap();
                    let request_message = ReadRequest::decode(message_bytes.as_slice()).unwrap();
                    // let event = request_message.
                    match (request_message.id, request_message.resource) {
                        (Some(id), Some(resource)) => {
                            if id != ID || resource != ID {
                                return RustResponse::default();
                            }
                        }
                        _ => return RustResponse::default(),
                    }

                    let response_message = HandleEffect {
                        effects: Effect { Render },
                    };
                    RustResponse {
                        successful: true,
                        message: Some(response_message.encode_to_vec()),
                        blob: None,
                    }
                }
                RustOperation::Update => RustResponse::default(),
                RustOperation::Delete => RustResponse::default(),
            }
        }

        pub fn view() -> ViewModel {
            todo_list::view(&MODEL.read().unwrap())
            // CORE.view(&MODEL)
        }
        pub fn stream_view() -> ViewModel {
            let mut view_model: ViewModel;
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                let signal_message = StateSignal { View_model };
                let rust_signal = RustSignal {
                    resource: ID,
                    message: Some(signal_message.encode_to_vec()),
                    blob: None,
                };
                send_rust_signal(rust_signal);
            }
        }
    }
}
