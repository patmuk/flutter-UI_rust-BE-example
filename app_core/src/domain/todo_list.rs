use crate::application::bridge::frb_generated::RustAutoOpaque;
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

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    AddTodo(String),
    RemoveTodo(u32),
    CleanList,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Query {
    AllTodos,
}

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
    RenderTodoList(RustAutoOpaque<TodoListModel>),
}

pub(crate) fn process_command_todo_list(
    command: Command,
    model: &RustAutoOpaque<TodoListModel>,
) -> Vec<Effect> {
    match command {
        Command::AddTodo(todo) => {
            model.blocking_write().items.push(TodoItem { text: (todo) });
            // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
            vec![Effect::RenderTodoList(model.clone())]
        }
        Command::RemoveTodo(todo_pos) => {
            model
                .blocking_write()
                .items
                .remove((todo_pos - 1).try_into().unwrap());
            vec![Effect::RenderTodoList(model.clone())]
        }
        Command::CleanList => {
            model.blocking_write().items.clear();
            vec![Effect::RenderTodoList(model.clone())]
        }
    }
}

pub(crate) fn process_query_todo_list(
    query: Query,
    model: &RustAutoOpaque<TodoListModel>,
) -> Vec<Effect> {
    match query {
        // the clone here is cheap, as it clones `RustAutoOpaque<T> = Arc<RwMutex<T>>`
        Query::AllTodos => vec![Effect::RenderTodoList(model.clone())],
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
    use super::*;

    #[test]
    fn add_todo() {
        let expected_model = RustAutoOpaque::new(TodoListModel {
            items: vec![TodoItem {
                text: String::from("test the list"),
            }],
        });
        let expected_effects = vec![Effect::RenderTodoList(expected_model.clone())];

        let mut actual_model = RustAutoOpaque::new(TodoListModel::default());
        let actual_effects =
            process_command_todo_list(Command::AddTodo("test the list".into()), &mut actual_model);

        assert_eq!(actual_effects, expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }

    #[test]
    fn remove_todo() {
        let expected_model = RustAutoOpaque::new(TodoListModel { items: vec![] });
        let expected_effect = Effect::RenderTodoList(expected_model.clone());

        let mut actual_model = RustAutoOpaque::new(TodoListModel {
            items: vec![TodoItem {
                text: "remove me".into(),
            }],
        });
        let actual_effects = process_command_todo_list(Command::RemoveTodo(1), &mut actual_model);

        assert_eq!(&actual_effects[0], &expected_effect);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }

    #[test]
    fn clean_list() {
        let expected_model = RustAutoOpaque::new(TodoListModel { items: vec![] });
        let expected_effects = vec![Effect::RenderTodoList(expected_model.clone())];

        let mut actual_model = RustAutoOpaque::new(TodoListModel::default());
        actual_model.blocking_write().items.push(TodoItem {
            text: "remove me".into(),
        });
        actual_model.blocking_write().items.push(TodoItem {
            text: "clean me".into(),
        });
        let actual_effects = process_command_todo_list(Command::CleanList, &mut actual_model);

        assert_eq!(&actual_effects, &expected_effects);
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }
}
