use crate::{
    domain::{
        common_value_objects::StateChanged,
        todo_list::{TodoListEffect, TodoListModelLock, TodoListProcessingError},
    },
    utils::cqrs_traits::Cqrs,
};

use super::{
    api_traits::{AppState, Lifecycle},
    lifecycle::LifecycleImpl,
};

///////////
// processing here to see codegen results
//////////
//TODO replace with macro_rules!([TodoComand, TodoQuery])
// TODO consider changing the name to TodoCommand_AddTodo
// test codegen for Dart!!!
#[derive(Debug)]
pub enum TodoCommand {
    AddTodo(String),
    RemoveTodo(usize),
    CleanList,
}
#[derive(Debug)]
pub enum TodoQuery {
    AllTodos,
}

/// All effects for the same reason all Processing Errors are in one emun:
/// - easier handling for the consumer (match)
/// - reuse of effects among domain implementations
#[derive(Debug)]
pub enum Effect {
    // Parameters need to be owned by `Effect`.
    // The attributes live in the app state - we don't want to
    // send them back and furth.
    // A reference is hard to manage - we could only `& mut` it when all
    // `&` are released, which only happens via `.dispose()` on the dart side.
    //
    // Thus, the best approach is providing a shared reference, which
    // Dart can not directly read: `RustAutoOpaque`, which is more or less a `Arc<RwLock>`.
    // Dart can access the lightweight properties needed for presentation with a function call
    // on this reference.
    //
    // In strict CQRS a command should not return a value.
    // However, this safes a consecutive query.
    // Thus, return only data for which a query exists.
    TodoListEffectRenderTodoList(TodoListModelLock),
}

impl Cqrs for TodoCommand {
    type Effect = Effect;
    type ProcessingError = ProcessingError;

    fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let lifecycle = LifecycleImpl::get();
        self.process_with_lifecycle(lifecycle)
    }
}

impl Cqrs for TodoQuery {
    type Effect = Effect;
    type ProcessingError = ProcessingError;

    fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        //todo get AppStateImpl for codegen -> from Lifecycle trait impl type
        self.process_with_lifecycle(LifecycleImpl::get())
    }
}

impl TodoQuery {
    fn process_with_lifecycle(
        self,
        lifecycle: &LifecycleImpl,
    ) -> Result<Vec<Effect>, ProcessingError> {
        let todo_list_model_lock = &lifecycle.app_state.todo_list_model_lock;
        let result = match self {
            TodoQuery::AllTodos => todo_list_model_lock.query_get_all_todos(),
        }
        .map_err(ProcessingError::TodoListProcessingError)?;
        Ok(result
            .into_iter()
            .map(|effect| match effect {
                TodoListEffect::RenderTodoList(model_lock) => {
                    Effect::TodoListEffectRenderTodoList(model_lock)
                }
            })
            .collect())
    }
}
impl TodoCommand {
    fn process_with_lifecycle(
        self,
        lifecycle: &LifecycleImpl,
    ) -> Result<Vec<Effect>, ProcessingError> {
        let app_state = &lifecycle.app_state;
        let todo_list_model_lock = &app_state.todo_list_model_lock;
        let (state_changed, result) = match self {
            TodoCommand::AddTodo(todo) => todo_list_model_lock.command_add_todo(todo),
            TodoCommand::RemoveTodo(todo_pos) => todo_list_model_lock.command_remove_todo(todo_pos),
            TodoCommand::CleanList => todo_list_model_lock.command_clean_list(),
        }
        .map_err(ProcessingError::TodoListProcessingError)?;
        if state_changed {
            app_state.mark_dirty();
            lifecycle.persist().map_err(ProcessingError::NotPersisted)?;
        }
        Ok(result
            .into_iter()
            .map(|effect| match effect {
                TodoListEffect::RenderTodoList(model_lock) => {
                    Effect::TodoListEffectRenderTodoList(model_lock)
                }
            })
            .collect())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ProcessingError {
    #[error("Error during processing: {0}")]
    TodoListProcessingError(TodoListProcessingError),
    #[error("Processing was fine, but state could not be persisted: {0}")]
    NotPersisted(#[source] std::io::Error),
}

/////////////
