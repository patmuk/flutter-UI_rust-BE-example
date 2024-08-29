use std::borrow::Borrow;

use log::debug;

use crate::application::api::lifecycle::{self, get_state};
use crate::domain::todo_list::{process_command_todo_list, process_query_todo_list};
pub use crate::domain::todo_list::{Command, Effect, Query};

// pub fn process_command<'a>(command: Command) -> Vec<Effect<'a>> {
pub fn process_command(command: Command) -> Result<Vec<Effect>, std::io::Error> {
    //&'static TodoListModel {
    debug!("Processing command: {:?}", command);
    let lifecycle = Lifecycle::get();
    let app_state = lifecycle.get_app_state();
    let model = &mut app_state.model;
    let effect = process_command_todo_list(command, model);
    // debug!("Processed command, new model {:?}", effect.first().unwrap()); // &model);
    debug!("Processed command, new model {:?}", &app_state.model);
    // TODO too much IO?
    lifecycle.persist_app_state();
    // lifecycle::persist_app_state(&app_state);
    effect
}

pub fn process_query(query: Query) -> Vec<Effect> {
    // pub fn process_query<'a>(query: Query) -> Vec<Effect<'a>> {
    //&model {
    debug!("Processing query: {:?}", query);
    // let effects = process_query_todo_list(query, &get_state().read().model);
    let effects = process_query_todo_list(query, &get_app_state().read().unwrap().model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
