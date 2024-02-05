use log::debug;

use crate::api::lifecycle::{self, API};
use crate::todo_list::process_mod_event;
pub use crate::todo_list::{Effect, Event};

#[derive(Debug, PartialEq, Eq)]
pub struct ViewModel {
    pub items: Vec<String>,
    pub count: usize,
}

pub fn process_event(event: Event) -> Vec<Effect> {
    debug!("Processing event: {:?}", event);
    let effects = process_mod_event(event, &mut API.write().model);
    debug!("Processed event, got the effects {:?}", effects);
    // TODO too much IO?
    lifecycle::persist_app_state(); //.unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
    effects
}

pub fn view() -> ViewModel {
    crate::todo_list::view(&API.read().model)
}
