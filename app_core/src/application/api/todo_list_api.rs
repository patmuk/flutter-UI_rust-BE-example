use log::debug;

use crate::application::api::lifecycle::APP_STATE;
pub use crate::domain::todo_list::Effect;
pub use crate::domain::todo_list::{Command, Query};
use crate::{
    application::api::lifecycle::get_app_state_ref,
    domain::todo_list::{process_command_todo_list, process_query_todo_list},
};
use crate::{
    application::api::lifecycle::{self},
    domain::app_state,
};

pub fn process_command(command: Command) -> Vec<Effect> {
    //&'static TodoListModel {
    debug!("Processing command: {:?}", command);
    // TODO maybe get static, so object instanciation is avoided
    // let app_state = &mut get_app_state_ref().lock.write();
    let mut app_state = APP_STATE
        .get()
        .expect("app state has been initialized")
        .write()
        .unwrap();
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
    // let effects = process_query_todo_list(query, &get_state().read().model);
    let effects = process_query_todo_list(
        query,
        &APP_STATE
            .get()
            .expect("app state has been initialized")
            .read()
            .unwrap()
            .model,
    );
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
