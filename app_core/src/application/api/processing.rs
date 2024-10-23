use std::io;

use super::lifecycle::Lifecycle;
use crate::{
    application::{app_state::AppState, bridge::frb_generated::RustAutoOpaque},
    domain::todo_list::{TodoListEffect, TodoListModel, TodoListProcessingError},
};

///////////
//// processing here to see codegen results
//////////
//TODO replace with macro_rules!([TodoComand, TodoQuery])
pub enum Cqrs {
    TodoCommandAddTodo(String),
    TodoCommandRemoveTodo(usize),
    TodoCommandCleanList,
    TodoQueryAllTodos,
}

// TODO codegen it

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
    TodoListEffectRenderTodoList(RustAutoOpaque<TodoListModel>),
    // TodoListEffectRenderTodoList(TodoListEffect),
}

impl Cqrs {
    pub(crate) fn process_with_app_state(
        self,
        app_state: &AppState,
    ) -> Result<Vec<Effect>, ProcessingError> {
        let result = match self {
            Cqrs::TodoCommandAddTodo(todo) => TodoListModel::add_todo(app_state, todo),
            Cqrs::TodoCommandRemoveTodo(todo_pos) => {
                TodoListModel::remove_todo(app_state, todo_pos)
            }
            Cqrs::TodoCommandCleanList => TodoListModel::clean_list(app_state),
            Cqrs::TodoQueryAllTodos => TodoListModel::get_all_todos(app_state),
        }
        .map_err(ProcessingError::TodoListProcessingError)?
        .into_iter()
        .map(|effect| match effect {
            TodoListEffect::RenderTodoList(content) => {
                Effect::TodoListEffectRenderTodoList(content)
            }
            _ => {
                panic!("Unknown effect");
            }
        })
        .collect();
        Ok(result)
    }
    pub fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let app_state = &Lifecycle::get().app_state;
        let result = self.process_with_app_state(app_state)?;
        //persist the state, but only if dirty
        let _ = app_state.persist().map_err(ProcessingError::NotPersisted);
        Ok(result)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ProcessingError {
    #[error("Error during processing: {0}")]
    TodoListProcessingError(TodoListProcessingError),
    #[error("Processing was fine, but state could not be persisted: {0}")]
    NotPersisted(#[source] io::Error),
}

/////////////
