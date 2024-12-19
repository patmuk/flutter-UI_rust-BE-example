use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};

use crate::application::api::lifecycle::{CqrsModel, CqrsModelLock};
use crate::application::bridge::frb_generated::RustAutoOpaque;

#[derive(Debug, Default, Clone)]
pub struct TodoListModelLock {
    pub lock: RustAutoOpaque<TodoListModel>,
}
impl CqrsModelLock<TodoListModel> for TodoListModelLock {}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TodoItem {
    pub text: String,
}

impl CqrsModel for TodoListModel {}

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
    /// this indicates that the model has changed, so that the app's state should be persisted.
    /// to avoid scanning the entire vec, this must be the first element.
    // this indicates that the model has changed, so that the app's state should be persisted.
    RenderTodoList(TodoListModelLock),
    RenderTodoItem(TodoItem),
}

impl From<TodoListModel> for TodoListModelLock {
    fn from(model: TodoListModel) -> Self {
        TodoListModelLock {
            lock: RustAutoOpaque::new(model),
        }
    }
}

impl From<TodoListModelLock> for TodoListModel {
    fn from(val: TodoListModelLock) -> Self {
        std::mem::take(&mut *val.lock.blocking_write())
    }
}

impl TodoListModelLock {
    /// as a command potentially changes the state, the bool denotes if a state change happened.
    pub(crate) fn command_add_todo(
        &self,
        todo: String,
    ) -> Result<(bool, Vec<TodoListEffect>), TodoListProcessingError> {
        self.lock
            .blocking_write()
            .items
            .push(TodoItem { text: todo });
        // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
        Ok((true, vec![TodoListEffect::RenderTodoList(self.clone())]))
    }

    pub(crate) fn command_remove_todo(
        &self,
        todo_pos: usize,
    ) -> Result<(bool, Vec<TodoListEffect>), TodoListProcessingError> {
        let items = &mut self.lock.blocking_write().items;
        if todo_pos > items.len() {
            Err(TodoListProcessingError::TodoDoesNotExist(todo_pos))
        } else {
            items.remove(todo_pos - 1);
            Ok((true, vec![TodoListEffect::RenderTodoList(self.clone())]))
        }
    }
    pub(crate) fn command_clean_list(
        &self,
    ) -> Result<(bool, Vec<TodoListEffect>), TodoListProcessingError> {
        self.lock.blocking_write().items.clear();
        Ok((true, vec![TodoListEffect::RenderTodoList(self.clone())]))
    }
    pub(crate) fn query_get_all_todos(
        &self,
    ) -> Result<Vec<TodoListEffect>, TodoListProcessingError> {
        Ok(vec![TodoListEffect::RenderTodoList(self.clone())])
    }
    pub(crate) fn query_get_todo(
        &self,
        todo_pos: usize,
    ) -> Result<Vec<TodoListEffect>, TodoListProcessingError> {
        let items = &self.lock.blocking_read().items;
        if todo_pos > items.len() {
            Err(TodoListProcessingError::TodoDoesNotExist(todo_pos))
        } else {
            let item = &items[todo_pos - 1];
            Ok(vec![TodoListEffect::RenderTodoItem(item.clone())])
        }
    }
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

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TodoListProcessingError {
    #[error("The todo at index {0} does not exist!")]
    TodoDoesNotExist(usize),
}

// only for tests, as the danger for a deadlock is too big
#[cfg(test)]
impl PartialEq for TodoListEffect {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TodoListEffect::RenderTodoList(own_model_lock),
                TodoListEffect::RenderTodoList(other_model_lock),
            ) => {
                // be aware of a potential deadlock here!
                // (lock on own, waiting for other and in another thread vice-versa!)
                let own_items = &own_model_lock.lock.blocking_read().items;

                let other_items = &other_model_lock.lock.blocking_read().items;
                own_items == other_items
            }
            (TodoListEffect::RenderTodoList(_), TodoListEffect::RenderTodoItem(_)) => false,
            (TodoListEffect::RenderTodoItem(_), TodoListEffect::RenderTodoList(_)) => false,
            (
                TodoListEffect::RenderTodoItem(todo_item),
                TodoListEffect::RenderTodoItem(other_todo_item),
            ) => todo_item == other_todo_item,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_model_lock(
        expected_model_lock: &TodoListModelLock,
        actual_model_lock: &TodoListModelLock,
    ) {
        assert_eq!(
            expected_model_lock.lock.blocking_read().items,
            actual_model_lock.lock.blocking_read().items
        );
    }

    #[test]
    fn add_todo() {
        let expected_model_lock: TodoListModelLock = TodoListModel {
            items: vec![TodoItem {
                text: String::from("test the list"),
            }],
        }
        .into();
        let expected_effects = (
            true,
            vec![TodoListEffect::RenderTodoList(expected_model_lock.clone())],
        );

        let actual_model_lock = TodoListModelLock::default();
        let actual_effects = actual_model_lock
            .command_add_todo("test the list".into())
            .unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq_model_lock(&expected_model_lock, &actual_model_lock);
    }

    #[test]
    fn remove_todo() {
        let expected_model_lock = TodoListModelLock::from(TodoListModel { items: vec![] });
        let expected_effects = (
            true,
            vec![TodoListEffect::RenderTodoList(expected_model_lock.clone())],
        );

        let actual_model = TodoListModel {
            items: vec![TodoItem {
                text: "remove me".into(),
            }],
        };
        let actual_model_lock: TodoListModelLock = actual_model.into();
        let actual_effects = actual_model_lock.command_remove_todo(1).unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq_model_lock(&expected_model_lock, &actual_model_lock);
        assert_eq!(
            Err(TodoListProcessingError::TodoDoesNotExist(1)),
            actual_model_lock.command_remove_todo(1),
        );
    }

    #[test]
    fn clean_list() {
        let expected_model_lock = TodoListModelLock::from(TodoListModel { items: vec![] });
        let expected_effects = (
            true,
            vec![TodoListEffect::RenderTodoList(expected_model_lock.clone())],
        );

        let mut actual_model = TodoListModel::default();
        actual_model.items.push(TodoItem {
            text: "remove me".into(),
        });
        actual_model.items.push(TodoItem {
            text: "clean me".into(),
        });
        let actual_model_lock: TodoListModelLock = actual_model.into();
        let actual_effects = actual_model_lock.command_clean_list().unwrap();

        assert_eq!(actual_effects, expected_effects);
        assert_eq_model_lock(&expected_model_lock, &actual_model_lock);
    }
}
