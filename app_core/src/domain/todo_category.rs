use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};

use crate::application::api::lifecycle::{CqrsModel, CqrsModelLock};
use crate::application::bridge::frb_generated::RustAutoOpaque;

use super::todo_list::TodoListModelLock;

#[derive(Debug, Clone)]
pub struct TodoCategoryModelLock {
    pub model: RustAutoOpaque<TodoCategoryModel>,
}
impl CqrsModelLock<TodoCategoryModel> for TodoCategoryModelLock {
    fn for_model(model: TodoCategoryModel) -> Self {
        Self {
            model: RustAutoOpaque::new(model),
        }
    }
}
impl Serialize for TodoCategoryModelLock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the model , the dirty flag is always false after loading
        self.model.blocking_read().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TodoCategoryModelLock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let model = TodoCategoryModel::deserialize(deserializer)?;
        Ok(Self::for_model(model))
    }
}

/// This is a simple Category for a todo-list, only to have a second model as an example.
/// Remember that each model should be a singleton, so that CRUD on the list of all instances
/// can be executed. (Otherwise a meta-model `categories` would be needed.)
/// For a real applicatiom, the model should hold a map of categories -> todo lists.
/// But for simplicity, we stick with one category only.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[frb(opaque)]
pub struct TodoCategoryModel {
    title: String,
    todo_list_lock: TodoListModelLock,
}

impl CqrsModel for TodoCategoryModel {}

impl Default for TodoCategoryModel {
    fn default() -> Self {
        Self {
            title: "TODOs".to_string(),
            todo_list_lock: TodoListModelLock::default(),
        }
    }
}

#[derive(Debug)]
pub enum TodoCategoryEffect {
    RenderTodoCategoryModel(TodoCategoryModelLock),
    RenderTodoCategory(String),
    RenderTodoList(TodoListModelLock),
    UpdateTitel(String),
}

impl TodoCategoryModelLock {
    /// as a command potentially changes the state, the bool denotes if a state change happened.
    pub(crate) fn update_title(
        &self,
        title: String,
    ) -> Result<(bool, Vec<TodoCategoryEffect>), TodoCategoryProcessingError> {
        self.model.blocking_write().title = title.clone();
        Ok((true, vec![TodoCategoryEffect::UpdateTitel(title)]))
    }

    pub(crate) fn get_todo_list(
        &self,
    ) -> Result<Vec<TodoCategoryEffect>, TodoCategoryProcessingError> {
        Ok(vec![TodoCategoryEffect::RenderTodoList(
            self.model.blocking_read().todo_list_lock.clone(),
        )])
    }
    pub(crate) fn set_todo_list(
        &self,
        todo_list_lock: TodoListModelLock,
    ) -> Result<(bool, Vec<TodoCategoryEffect>), TodoCategoryProcessingError> {
        self.model.blocking_write().todo_list_lock = todo_list_lock.clone();
        Ok((
            true,
            vec![TodoCategoryEffect::RenderTodoList(todo_list_lock)],
        ))
    }
    pub(crate) fn get_todo_category_model(
        &self,
    ) -> Result<Vec<TodoCategoryEffect>, TodoCategoryProcessingError> {
        Ok(vec![TodoCategoryEffect::RenderTodoCategoryModel(
            self.clone(),
        )])
    }
    pub(crate) fn get_todo_category(
        &self,
        get_model: bool,
    ) -> Result<Vec<TodoCategoryEffect>, TodoCategoryProcessingError> {
        if get_model {
            self.get_todo_category_model()
        } else {
            Ok(vec![TodoCategoryEffect::RenderTodoCategory(
                self.model.blocking_read().title.clone(),
            )])
        }
    }
}

impl TodoCategoryModel {
    /// This is how to access the fields of a heavy object behind a RustAutoOpaque.
    /// This is copying parts the content, which Dart needs to show to the user.
    ///
    /// If `items` would be `pub` FRB would automatically create a getter. However, this
    /// getter would `clone()` the `items` as well. As we pretend that a single item
    /// is heavy to clone, we use a custom function to `clone()` only the lightweight and
    /// only needed part for presentation.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_todo_list_lock(&self) -> TodoListModelLock {
        self.todo_list_lock.clone()
    }
    pub fn get_todos(&self) -> Vec<String> {
        self.todo_list_lock
            .model
            .blocking_read()
            .get_todos_as_string()
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TodoCategoryProcessingError {
    #[error("The todo at index {0} does not exist!")]
    TodoDoesNotExist(usize),
}

#[cfg(test)]
mod tests {
    use crate::domain::todo_list::TodoListModel;

    use super::*;

    #[test]
    fn test_update_title() {
        let model_lock = TodoCategoryModelLock::for_model(TodoCategoryModel {
            title: String::from("Initial Title"),
            todo_list_lock: TodoListModelLock::for_model(TodoListModel::default()),
        });

        // update the title
        let result = model_lock.update_title(String::from("New Title"));
        assert!(result.is_ok());
        let (updated, effects) = result.unwrap();
        assert!(updated);
        assert_eq!(1, effects.len());
        match &effects[0] {
            TodoCategoryEffect::UpdateTitel(title) => {
                assert_eq!("New Title", title);
            }
            _ => panic!("unexpected effect"),
        }

        // check if the title was updated
        assert_eq!(
            String::from("New Title"),
            model_lock.model.blocking_read().title
        );
    }

    #[test]
    fn test_get_todo_list() {
        let todo_list_lock = TodoListModelLock::for_model(TodoListModel::default());
        let _ = todo_list_lock.command_add_todo("test getting all todos".to_string());
        let _ = todo_list_lock.command_add_todo("test the second todo".to_string());

        let model_lock = TodoCategoryModelLock::for_model(TodoCategoryModel {
            title: String::from("Initial Category"),
            todo_list_lock,
        });

        // get the todo list
        let result = model_lock.get_todo_list();
        assert!(result.is_ok());
        let effects = result.unwrap();
        assert_eq!(1, effects.len());

        match &effects[0] {
            TodoCategoryEffect::RenderTodoList(todo_list_model_lock) => {
                let todo_list = todo_list_model_lock
                    .model
                    .blocking_read()
                    .get_todos_as_string();
                assert_eq!(2, todo_list.len());
                assert_eq!("test getting all todos", todo_list[0]);
                assert_eq!("test the second todo", todo_list[1]);
            }
            _ => panic!("unexpected effect"),
        }

        // check if the title was updated
        let actual_todos = model_lock
            .model
            .blocking_read()
            .todo_list_lock
            .model
            .blocking_read()
            .get_todos_as_string();
        assert_eq!(2, actual_todos.len());
        assert_eq!("test getting all todos", actual_todos[0]);
        assert_eq!("test the second todo", actual_todos[1]);
    }

    #[test]
    fn test_set_todo_list() {
        let todo_list_lock = TodoListModelLock::for_model(TodoListModel::default());
        let _ = todo_list_lock.command_add_todo("test getting all todos".to_string());
        let _ = todo_list_lock.command_add_todo("test the second todo".to_string());

        let model_lock = TodoCategoryModelLock::for_model(TodoCategoryModel::default());

        // set the todo list
        let result = model_lock.set_todo_list(todo_list_lock);
        assert!(result.is_ok());
        let (status_changed, effects) = result.unwrap();
        assert!(status_changed);
        assert_eq!(1, effects.len());

        match &effects[0] {
            TodoCategoryEffect::RenderTodoList(todo_list_model_lock) => {
                let todo_list = todo_list_model_lock
                    .model
                    .blocking_read()
                    .get_todos_as_string();
                assert_eq!(2, todo_list.len());
                assert_eq!("test getting all todos", todo_list[0]);
                assert_eq!("test the second todo", todo_list[1]);
            }
            _ => panic!("unexpected effect"),
        }

        // check if the title was updated
        let actual_todos = model_lock
            .model
            .blocking_read()
            .todo_list_lock
            .model
            .blocking_read()
            .get_todos_as_string();
        assert_eq!(2, actual_todos.len());
        assert_eq!("test getting all todos", actual_todos[0]);
        assert_eq!("test the second todo", actual_todos[1]);
    }

    #[test]
    fn test_get_todo_category() {
        let todo_list_lock = TodoListModelLock::for_model(TodoListModel::default());
        let _ = todo_list_lock.command_add_todo("test getting all todos".to_string());
        let _ = todo_list_lock.command_add_todo("test the second todo".to_string());

        let model_lock = TodoCategoryModelLock::for_model(TodoCategoryModel {
            title: String::from("Initial Category"),
            todo_list_lock,
        });

        // get the todo list
        let result = model_lock.get_todo_category(false);
        assert!(result.is_ok());
        let effects = result.unwrap();
        assert_eq!(1, effects.len());

        match &effects[0] {
            TodoCategoryEffect::RenderTodoCategory(todo_category) => {
                assert_eq!("Initial Category", todo_category);
            }
            _ => panic!("unexpected effect"),
        }
        // check if the title was updated
        let actual_todo_category = &model_lock.model.blocking_read().title;
        assert_eq!("Initial Category", actual_todo_category);
    }

    #[test]
    fn test_get_category_model() {
        let todo_list_lock = TodoListModelLock::for_model(TodoListModel::default());
        let _ = todo_list_lock.command_add_todo("test getting all todos".to_string());
        let _ = todo_list_lock.command_add_todo("test the second todo".to_string());

        let model_lock = TodoCategoryModelLock::for_model(TodoCategoryModel {
            title: String::from("Initial Category"),
            todo_list_lock,
        });

        // get the todo list
        let result = model_lock.get_todo_category_model();
        assert!(result.is_ok());
        let effects = result.unwrap();
        assert_eq!(1, effects.len());

        match &effects[0] {
            TodoCategoryEffect::RenderTodoCategoryModel(todo_category_model_lock) => {
                let todo_category = todo_category_model_lock.model.blocking_read().get_title();
                assert_eq!("Initial Category", todo_category);
            }
            _ => panic!("unexpected effect"),
        }

        // check if the title was updated
        let actual_todo_category = model_lock.model.blocking_read().get_title();
        assert_eq!("Initial Category", actual_todo_category);
    }
}
