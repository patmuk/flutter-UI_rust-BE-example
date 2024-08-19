use log::debug;

pub use crate::domain::todo_list::Effect;
use crate::domain::todo_list::{process_command_todo_list, process_query_todo_list};
pub use crate::domain::todo_list::{Command, Query};
use crate::{
    application::api::lifecycle::{self, get_state},
    domain::app_state,
};

pub fn process_command(command: Command) -> Vec<Effect> {
    //&'static TodoListModel {
    debug!("Processing command: {:?}", command);
    let app_state = &mut get_state().write();
    let model = &mut app_state.model;
    let effect = process_command_todo_list(command, model);
    debug!("Processed command, new model {:?}", model);
    // TODO too much IO?
    lifecycle::persist_app_state(&app_state);
    effect
}

pub fn process_query(query: Query) -> Vec<Effect> {
    //&model {
    debug!("Processing query: {:?}", query);
    let effects = process_query_todo_list(query, &get_state().read().model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
