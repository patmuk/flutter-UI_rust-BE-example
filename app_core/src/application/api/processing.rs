use crate::application::bridge::frb_generated::RustAutoOpaque;
pub use crate::domain::todo_list::TodoListModel;
use crate::domain::todo_list::{TodoCommand, TodoEffect, TodoProcessingError, TodoQuery};
use crate::utils::cqrs_traits::{Cqrs, CqrsModel, Effect};
use crate::{application::api::lifecycle::Lifecycle, utils::cqrs_traits::ProcessingError};
use log::debug;

pub fn process_todo_model_command(
    command: TodoCommand,
) -> Result<Vec<TodoEffect>, TodoProcessingError> {
    process_command(command, &Lifecycle::get().app_state.model)
}
pub fn process_todo_model_query(query: TodoQuery) -> Result<Vec<TodoEffect>, TodoProcessingError> {
    process_query(query, &Lifecycle::get().app_state.model)
}

fn process_command<M: CqrsModel, E: Effect, PE: ProcessingError>(
    command: impl Cqrs<Model = M, Effect = E, ProcessingError = PE> + 'static,
    model: &RustAutoOpaque<M>,
) -> Result<Vec<E>, PE>
where
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<M>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    debug!("Processing command: {:?}", command);
    let lifecycle = Lifecycle::get();
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

fn process_query<M: CqrsModel, E: Effect, PE: ProcessingError>(
    query: impl Cqrs<Model = M, Effect = E, ProcessingError = PE> + 'static,
    model: &RustAutoOpaque<M>,
) -> Result<Vec<E>, PE>
where
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<M>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    debug!("Processing query: {:?}", query);
    let effects = query.process(model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
