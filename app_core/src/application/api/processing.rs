use crate::application::bridge::frb_generated::RustAutoOpaque;
pub use crate::domain::todo_list::TodoListModel;
use crate::domain::todo_list::{TodoEffect, TodoProcessingError};
use crate::utils::cqrs_traits::{Cqrs, Effect};
use crate::{application::api::lifecycle::Lifecycle, utils::cqrs_traits::ProcessingError};
use log::debug;

pub fn process_todo_command(
    command: impl Cqrs<Model = RustAutoOpaque<TodoListModel>> + 'static,
) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // process(command, Lifecycle::get().app_state.model.clone())
    process_command(command, &Lifecycle::get().app_state.model)
}

fn process_command<M>(
    // fn process<M>(
    command: impl Cqrs<Model = M> + 'static,
    model: &'static M,
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
) -> Result<Vec<impl Effect>, impl ProcessingError + '_> {
    // pub fn process_command(command: impl Cqrs) -> Result<Vec<impl Effect>, impl ProcessingError> {
    debug!("Processing command: {:?}", command);
    let lifecycle = Lifecycle::get();
    // let app_state = &mut lifecycle.app_state.write().unwrap();
    let effects = command.process(model);
    debug!(
        "Processed command, new model {:?}",
        lifecycle.app_state.model.blocking_read()
    );
    lifecycle.app_state.mark_dirty();
    // persist change to not miss it
    lifecycle
        .app_state
        .persist(&lifecycle.app_config.app_state_file_path)
        .unwrap(); // TODO convert to own error
                   // Ok::<_, dyn ProcessingError>(effects)
    effects
}

pub fn process_query(
    query: impl Cqrs<Model = RustAutoOpaque<TodoListModel>>,
) -> Result<Vec<impl Effect>, impl ProcessingError> {
    debug!("Processing query: {:?}", query);
    let effects = query.process(&Lifecycle::get().app_state.model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
