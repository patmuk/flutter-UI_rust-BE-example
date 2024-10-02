use crate::application::api::lifecycle::Lifecycle;
pub use crate::domain::todo_list::Effect;
use crate::domain::todo_list::{process_command_todo_list, process_query_todo_list};
use log::debug;

pub use crate::domain::todo_list::{Command, Query};

pub fn process_command(command: Command) -> Result<Vec<Effect>, std::io::Error> {
    debug!("Processing command: {:?}", command);
    let lifecycle = Lifecycle::get();
    let app_state = &mut lifecycle.app_state.write().unwrap();
    let effect = process_command_todo_list(command, &mut app_state.model);
    debug!("Processed command, new model {:?}", app_state.model);
    // persist change to not miss it
    app_state.persist(&lifecycle.app_config.app_state_file_path)?;
    Ok(effect)
}

pub fn process_query(query: Query) -> Vec<Effect> {
    debug!("Processing query: {:?}", query);
    let effects = process_query_todo_list(query, &Lifecycle::get().app_state.read().unwrap().model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
