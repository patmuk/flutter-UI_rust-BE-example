use log::debug;

use crate::application::api::lifecycle::{self, API};
use crate::domain::todo_list::{process_command_todo_list, process_query_todo_list};
pub use crate::domain::todo_list::{Command, Effect, Query};

pub fn process_command(command: Command) -> Vec<Effect> {
    debug!("Processing command: {:?}", command);
    let effects = process_command_todo_list(command, &mut API.write().model);
    debug!("Processed command, got the effects {:?}", effects);
    // TODO too much IO?
    lifecycle::persist_app_state(); //.unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
    effects
}

pub fn process_query(query: Query) -> Vec<Effect> {
    debug!("Processing query: {:?}", query);
    let effects = process_query_todo_list(query);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
