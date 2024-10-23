use crate::{
    application::{app_state::AppState, bridge::frb_generated::RustAutoOpaque},
    utils::cqrs_traits::CqrsModel,
};
use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum TodoListEffect {
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
    RenderTodoList(RustAutoOpaque<TodoListModel>),
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

    pub(crate) fn add_todo(
        app_state: &AppState,
        todo: String,
    ) -> Result<Vec<TodoListEffect>, TodoListProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        model_lock
            .blocking_write()
            .items
            .push(TodoItem { text: todo });
        app_state.mark_dirty();
        // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
        Ok(vec![TodoListEffect::RenderTodoList(model_lock.clone())])
    }
    pub(crate) fn remove_todo(
        app_state: &AppState,
        todo_pos: usize,
    ) -> Result<Vec<TodoListEffect>, TodoListProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        let items = &mut model_lock.blocking_write().items;
        if todo_pos > items.len() {
            Err(TodoListProcessingError::TodoDoesNotExist(todo_pos))
        } else {
            items.remove(todo_pos - 1);
            app_state.mark_dirty();
            Ok(vec![TodoListEffect::RenderTodoList(model_lock.clone())])
        }
    }
    pub(crate) fn clean_list(
        app_state: &AppState,
    ) -> Result<Vec<TodoListEffect>, TodoListProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        model_lock.blocking_write().items.clear();
        app_state.mark_dirty();
        Ok(vec![TodoListEffect::RenderTodoList(model_lock.clone())])
    }
    pub(crate) fn get_all_todos(
        app_state: &AppState,
    ) -> Result<Vec<TodoListEffect>, TodoListProcessingError> {
        let model_lock = TodoListModel::get_model_lock(app_state);
        Ok(vec![TodoListEffect::RenderTodoList(model_lock.clone())])
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TodoListProcessingError {
    #[error("The todo at index {0} does not exist!")]
    TodoDoesNotExist(usize),
}

impl CqrsModel for TodoListModel {
    /// bootstrap the model from the app's state
    fn get_model_lock(app_state: &AppState) -> &RustAutoOpaque<Self> {
        &app_state.model
    }
}

// only for tests, as the danger for a deadlock is too big
#[cfg(test)]
impl PartialEq for TodoListEffect {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (TodoListEffect::RenderTodoList(own), TodoListEffect::RenderTodoList(other)) if {
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
    use crate::application::app_state::AppState;

    use super::*;

    #[test]
    fn add_todo() {
        let expected_model = RustAutoOpaque::new(TodoListModel {
            items: vec![TodoItem {
                text: String::from("test the list"),
            }],
        });
        let expected_effects = vec![TodoListEffect::RenderTodoList(expected_model.clone())];

        let actual_model = RustAutoOpaque::new(TodoListModel::default());
        let app_state = AppState::from_model(&actual_model);
        let actual_effects = TodoListModel::add_todo(&app_state, "test the list".into()).unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }

    #[test]
    fn remove_todo() {
        let expected_model = RustAutoOpaque::new(TodoListModel { items: vec![] });
        let expected_effects = vec![TodoListEffect::RenderTodoList(expected_model.clone())];

        let actual_model = RustAutoOpaque::new(TodoListModel {
            items: vec![TodoItem {
                text: "remove me".into(),
            }],
        });
        let app_state = AppState::from_model(&actual_model);
        let actual_effects = TodoListModel::remove_todo(&app_state, 1).unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
        assert_eq!(
            TodoListModel::remove_todo(&app_state, 1),
            Err(TodoListProcessingError::TodoDoesNotExist(1))
        );
    }

    #[test]
    fn clean_list() {
        let expected_model = RustAutoOpaque::new(TodoListModel { items: vec![] });
        let expected_effects = vec![TodoListEffect::RenderTodoList(expected_model.clone())];

        let actual_model = RustAutoOpaque::new(TodoListModel::default());
        actual_model.blocking_write().items.push(TodoItem {
            text: "remove me".into(),
        });
        actual_model.blocking_write().items.push(TodoItem {
            text: "clean me".into(),
        });
        let app_state = AppState::from_model(&actual_model);

        let actual_effects = TodoListModel::clean_list(&app_state).unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }
}
