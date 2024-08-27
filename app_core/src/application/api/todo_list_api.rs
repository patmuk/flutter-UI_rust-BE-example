use std::borrow::Borrow;

use log::debug;

pub use crate::domain::todo_list::Effect;
use crate::domain::todo_list::{process_command_todo_list, process_query_todo_list};
pub use crate::domain::todo_list::{Command, Query};
use crate::{
    application::api::lifecycle::{self, get_app_state},
    domain::app_state,
};

pub fn process_command<'a>(command: Command) -> Vec<Effect<'a>> {
    //&'static TodoListModel {
    debug!("Processing command: {:?}", command);
    let mut app_state = get_app_state().write().unwrap();
    let model = &mut app_state.model;
    let effect = process_command_todo_list(command, model);
    // debug!("Processed command, new model {:?}", effect.first().unwrap()); // &model);
    let app_state = get_app_state().read().unwrap();
    debug!("Processed command, new model {:?}", &app_state.model);
    // TODO too much IO?
    lifecycle::persist_app_state(app_state.borrow());
    // lifecycle::persist_app_state(&app_state);
    effect
}

pub fn process_query<'a>(query: Query) -> Vec<Effect<'a>> {
    //&model {
    debug!("Processing query: {:?}", query);
    // let effects = process_query_todo_list(query, &get_state().read().model);
    let effects = process_query_todo_list(query, &get_app_state().read().unwrap().model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
