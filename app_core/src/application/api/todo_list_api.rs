use log::debug;

use crate::application::api::lifecycle::{self, get_state};
use crate::domain::todo_list::{
    process_command_todo_list, process_query_todo_list, Effect, TodoListModel,
};
pub use crate::domain::todo_list::{Command, Query};

pub fn process_command(command: Command) -> Vec<Effect> {
    //&'static TodoListModel {
    debug!("Processing command: {:?}", command);
    let model = &mut get_state().write().model;
    let effect = process_command_todo_list(command, model);
    debug!("Processed command, new model {:?}", model);
    // TODO too much IO?
    lifecycle::persist_app_state(); //.unwrap_or_else(|err| error!("Error persisting app state: {:?}", err));
                                    // model
    effect
}

pub fn process_query(query: Query) -> Vec<Effect> {
    //&model {
    debug!("Processing query: {:?}", query);
    let effects = process_query_todo_list(query, &get_state().read().model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
