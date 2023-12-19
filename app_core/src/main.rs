// initializes and logs a test message using OS logger
use app_core::api::{self, Effect, AppConfig};

fn main() {
    // Send a test log message
    log::info!("Executing the example code.");

    // app.init() is automatically called on first access to api.
    // However, you can call it manually if you want to trigger the initialization.
    // As this attempts to load the app state from disk, it could take a while for IO.
    
    // optionally set up the file path to store the app state
    api::setup(AppConfig{app_state_file_path: "./test_app_state.bin"}).unwrap();

    api::process_event(api::Event::AddTodo("Test todo".to_string()));

    let view = api::view();
    println!("Todos: {:?}", view.items);

    // following the crux-style, one should not call view() directly, but evaluate the Effect, which tells
    // the caller ('shell') what to do (in this case render the viewModel):
    let effects = api::process_event(api::Event::AddTodo("Proper Test todo".to_string()));
    for effect in effects {
        match effect {
            Effect::Render(view) => {
                println!("Rendering view: {:?}", view);
            }
        }
    }
    // if possible call this function to clean up, like wrting the app's state to disk
    api::shutdown();
}
