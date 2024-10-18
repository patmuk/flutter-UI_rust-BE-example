use super::lifecycle::{Effect, Lifecycle, ProcessingError};
use crate::{application::app_state::AppState, domain::todo_list::TodoListModel};

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

impl Cqrs {
    pub(crate) fn process_with_app_state(
        self,
        app_state: &AppState,
    ) -> Result<Vec<Effect>, ProcessingError> {
        match self {
            Cqrs::TodoCommandAddTodo(todo) => TodoListModel::add_todo(app_state, todo),
            Cqrs::TodoCommandRemoveTodo(todo_pos) => {
                TodoListModel::remove_todo(app_state, todo_pos)
            }
            Cqrs::TodoCommandCleanList => TodoListModel::clean_list(app_state),
            Cqrs::TodoQueryAllTodos => TodoListModel::get_all_todos(app_state),
        }
    }
    pub fn process(self) -> Result<Vec<Effect>, ProcessingError> {
        let app_state = &Lifecycle::get().app_state;
        let result = self.process_with_app_state(app_state);
        //persist the state, but only if dirty
        //TODO handle persistence error
        app_state.persist();
        result
    }
}
/////////////
