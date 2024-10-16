use crate::{
    application::bridge::frb_generated::RustAutoOpaque,
    utils::cqrs_traits::{Cqrs, CqrsModel},
};
use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};

use super::{effects::Effect, processing_errors::ProcessingError};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[frb(opaque)]
pub struct TodoListModel {
    /// the vector 'items' is the critical resource we want to protect for concurrent access.
    /// RustAutoOpaque<T> translates (simplified) to Arc<RwLock<T>> and thus can safely be sent between threads.
    /// Note that you can go more corse-grain (meaning wrapping the whole TodoListModel or even AppState)
    /// but the finer the better performance you will get (as the lock is hold for a shorter time).
    /// As a roule of thumb, wrap all fields in a RustAutoOpaque, that should be written to in an atomic action.
    ///
    /// You can wrap multiple fields in RustAutoOpaque's, but you cannot wrap sub-fields.
    /// Besides a compilation error (the trait `application::bridge::frb_generated::MoiArcValue` is not implemented for)
    /// you could easily run into a deadlock.
    /// As we cannot implement a function on `Vec` which would give use the content of a TodoItem, we wrapped
    /// the `TodoListModel` in `RustAutoOpaque`
    items: Vec<TodoItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TodoItem {
    text: String,
}

impl TodoListModel {
    /// This is how to access the fields of a heavy object behind a RustAutoOpaque.
    /// This is copying parts the content, which Dart needs to show to the user.
    ///
    /// If `items` would be `pub` FRB would automatically create a getter. However, this
    /// getter would `clone()` the `items` as well. As we pretend that a single item
    /// is heavy to clone, we use a custom function to `clone()` only the lightweight and
    /// only needed part for presentation.
    pub fn get_todos_as_string(&self) -> Vec<String> {
        self.items.iter().map(|item| item.text.clone()).collect()
    }
}
impl CqrsModel for TodoListModel {
    fn get_model(app_state: &super::app_state::AppState) -> &RustAutoOpaque<Self> {
        &app_state.model
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TodoCommand {
    AddTodo(String),
    RemoveTodo(usize),
    CleanList,
}

// impl TodoCommand for TodoCommand {}
impl Cqrs for TodoCommand {
    fn process(
        self,
        app_state: &super::app_state::AppState,
    ) -> Result<Vec<Effect>, ProcessingError> {
        let model = TodoListModel::get_model(&app_state);
        match self {
            TodoCommand::AddTodo(todo) => {
                model.blocking_write().items.push(TodoItem { text: todo });
                // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
                Ok(vec![Effect::RenderTodoList(model.clone())])
            }
            TodoCommand::RemoveTodo(todo_pos) => {
                let items = &mut model.blocking_write().items;
                if todo_pos > items.len() {
                    Err(ProcessingError::TodosDoesNotExist(todo_pos))
                } else {
                    items.remove(todo_pos - 1);
                    Ok(vec![Effect::RenderTodoList(model.clone())])
                }
            }
            TodoCommand::CleanList => {
                model.blocking_write().items.clear();
                Ok(vec![Effect::RenderTodoList(model.clone())])
            }
        }
    }
    fn is_command(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TodoQuery {
    AllTodos,
}

impl Cqrs for TodoQuery {
    fn process(
        self,
        app_state: &super::app_state::AppState,
    ) -> Result<Vec<Effect>, ProcessingError> {
        let model = TodoListModel::get_model(&app_state);
        Ok::<std::vec::Vec<Effect>, ProcessingError>(match self {
            // the clone here is cheap, as it clones `RustAutoOpaque<T> = Arc<RwMutex<T>>`
            TodoQuery::AllTodos => vec![Effect::RenderTodoList(model.clone())],
        })
    }
    fn is_command(&self) -> bool {
        false
    }
}

// only for tests, as the danger for a deadlock is too big
#[cfg(test)]
impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (Effect::RenderTodoList(own), Effect::RenderTodoList(other)) if {
            // be aware of a potential deadlock here!
            // (lock on own, waiting for other and in another thread vice-versa!)
            let own_items = &own.blocking_read().items;
            let other_items = &other.blocking_read().items;
            own_items == other_items
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::app_state::AppState;

    use super::*;

    #[test]
    fn add_todo() {
        let expected_model = RustAutoOpaque::new(TodoListModel {
            items: vec![TodoItem {
                text: String::from("test the list"),
            }],
        });
        let expected_effects = vec![Effect::RenderTodoList(expected_model.clone())];

        let actual_model = RustAutoOpaque::new(TodoListModel::default());
        let app_state = AppState::from_model(&actual_model);
        let actual_effects = TodoCommand::AddTodo("test the list".into())
            .process(&app_state)
            .unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }

    #[test]
    fn remove_todo() {
        let expected_model = RustAutoOpaque::new(TodoListModel { items: vec![] });
        let expected_effects = vec![Effect::RenderTodoList(expected_model.clone())];

        let actual_model = RustAutoOpaque::new(TodoListModel {
            items: vec![TodoItem {
                text: "remove me".into(),
            }],
        });
        let app_state = AppState::from_model(&actual_model);
        let actual_effects = TodoCommand::RemoveTodo(1).process(&app_state).unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
        assert_eq!(
            TodoCommand::RemoveTodo(1).process(&app_state),
            Err(ProcessingError::TodosDoesNotExist(1))
        );
    }

    #[test]
    fn clean_list() {
        let expected_model = RustAutoOpaque::new(TodoListModel { items: vec![] });
        let expected_effects = vec![Effect::RenderTodoList(expected_model.clone())];

        let actual_model = RustAutoOpaque::new(TodoListModel::default());
        actual_model.blocking_write().items.push(TodoItem {
            text: "remove me".into(),
        });
        actual_model.blocking_write().items.push(TodoItem {
            text: "clean me".into(),
        });
        let app_state = AppState::from_model(&actual_model);

        let actual_effects = TodoCommand::CleanList.process(&app_state).unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }
}
