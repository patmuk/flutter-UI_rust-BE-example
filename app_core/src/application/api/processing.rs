use crate::application::bridge::frb_generated::RustAutoOpaque;
pub use crate::domain::todo_list::TodoListModel;
use crate::domain::todo_list::{TodoCommand, TodoEffect, TodoProcessingError, TodoQuery};
use crate::utils::cqrs_traits::{Cqrs, CqrsModel, Effect};
use crate::{application::api::lifecycle::Lifecycle, utils::cqrs_traits::ProcessingError};
use log::debug;

// fn process(
//     // command: TodoCommand,
//     command: impl Cqrs<Model = TodoListModel> + 'static,
//     // command: impl Cqrs<Model = TodoListModel, Effect = TodoEffect, ProcessingError = TodoProcessingError>
//     //     + 'static,
// ) -> Result<Vec<impl Effect>, impl ProcessingError> {
//     // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
//     // process(command, Lifecycle::get().app_state.model.clone())
//     match command {
//         TodoCommand => process_command(command, &Lifecycle::get().app_state.model),
//         TodoQuery => process_query(command, &Lifecycle::get().app_state.model),
//     }
// }
pub fn process_todo_model_command(
    command: TodoCommand,
    // command: impl Cqrs<Model = TodoListModel, Effect = TodoEffect, ProcessingError = TodoProcessingError>
    // + 'static,
) -> Result<Vec<TodoEffect>, TodoProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // process(command, Lifecycle::get().app_state.model.clone())
    process_command(command, &Lifecycle::get().app_state.model)
    // match command {
    //     TodoCommand => process_command(command, &Lifecycle::get().app_state.model),
    //     TodoQuery => process_query(command, &Lifecycle::get().app_state.model),
    // }
}
pub fn process_todo_model_query(
    query: TodoQuery, //impl Cqrs<Model = RustAutoOpaque<TodoListModel>> + 'static,
) -> Result<Vec<TodoEffect>, TodoProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // process(command, Lifecycle::get().app_state.model.clone())
    process_query(query, &Lifecycle::get().app_state.model)
}

fn process_command<M: CqrsModel, E: Effect, PE: ProcessingError>(
    // fn process<M>(
    command: impl Cqrs<Model = M, Effect = E, ProcessingError = PE> + 'static,
    // model: &'static M,
    model: &RustAutoOpaque<M>,
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError + '_> {
) -> Result<Vec<E>, PE>
where
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<M>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    // fn process_command(
    //     // fn process<M>(
    //     command: impl Cqrs<
    //         Model = TodoListModel,
    //         Effect = TodoEffect,
    //         ProcessingError = TodoProcessingError,
    //     >,
    //     model: &RustAutoOpaque<TodoListModel>,
    // ) -> Result<Vec<TodoEffect>, TodoProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError + '_> {
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

fn process_query<M: CqrsModel, E: Effect, PE: ProcessingError>(
    // fn process<M>(
    query: impl Cqrs<Model = M, Effect = E, ProcessingError = PE> + 'static,
    // model: &'static M,
    model: &RustAutoOpaque<M>,
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    // ) -> Result<Vec<impl Effect>, impl ProcessingError + '_> {
) -> Result<Vec<E>, PE>
where
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<M>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    // fn process_query<M>(
    // query: impl Cqrs<Model = RustAutoOpaque<TodoListModel>>,
    // query: impl Cqrs<Model = M> + 'static,
    // model: &'static M,
    // ) -> Result<Vec<impl Effect>, impl ProcessingError> {
    debug!("Processing query: {:?}", query);
    let effects = query.process(model);
    debug!("Processed query, got the effects {:?}", effects);
    effects
}
