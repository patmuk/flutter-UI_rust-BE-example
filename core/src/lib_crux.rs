use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::api::{self, TodoListModel};
pub use crate::api::{Effect, Event, ViewModel};

//pub use crux_core::Request;
// use crux_core::{App, Core};

// TODO hide this plumbing

//uniffi::include_scaffolding!("shared");

lazy_static! {
// //     // static ref CORE: Core<Effect, TodoList> = Core::new::<Capabilities>();
// //     // static ref CORE : TodoList = TodoList {};
//     // static ref MODEL: TodoListModel = TodoListModel{..Default::default()};
    // static ref MODEL: RwLock<TodoListModel> = RwLock::new(TodoListModel{..Default::default()});
    static ref MODEL: RwLock<TodoListModel> = RwLock::new(TodoListModel::default());
}

pub fn process_event(event: Event) -> Vec<Effect> {
    //CORE.process_event(data)
    match event {
        // Event::AddTodo(_) => CORE.update(event, &mut MODEL),
        Event::AddTodo(_) => api::update(event, &mut MODEL.write().unwrap()),
        Event::RemoveTodo(_) => api::update(event, &mut MODEL.write().unwrap()),
        Event::CleanList => api::update(event, &mut MODEL.write().unwrap()),
    }
}

// pub fn handle_response(uuid: &[u8], data: &[u8]) -> Vec<u8> {
//     // CORE.handle_response(uuid, data)
//     todo!();
// }

pub fn view() -> ViewModel {
    api::view(&MODEL.read().unwrap())
    // CORE.view(&MODEL)
}
// pub fn view() -> Vec<u8> {
//     CORE.view()
// }
