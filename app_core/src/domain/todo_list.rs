use crate::application::bridge::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
use serde::{ser::SerializeSeq, Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
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
    pub items: Vec<TodoItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub text: String,
}

impl TodoItem {
    // this is how to access the fields of a heavy object behind a RustAutoOpaque.
    pub fn get_text(&self) -> &str {
        &self.text
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
    // TODO check if still correct and rewrite
    //
    // Parameters need to be owned - as they live inside the app state.
    // To send them between threads and ro Dart, we need to wrap them into an
    // RustAutoOpaque<T>, which is an Arc<RwMutex<T>>.
    // Thus, we clone the Arc when passing it into the Effect.
    // FRB takes care of aquiring the lock to read and releases it
    // when the Dart Garbage Collector disposes the object.
    // To force this, call dispose() on the object in Dart manually.
    //
    // In strict CQRS a command should not return a value.
    // However, this safes a consecutive query.
    // Thus, return only data for which a query exists.
    RenderTodoList(RustAutoOpaque<TodoListModel>),
}

pub(crate) fn process_command_todo_list(
    command: Command,
    model: &mut RustAutoOpaque<TodoListModel>,
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
            // TODO remove comments below once it is confirmed that == test the vecs content
        //     (own.items.len() == other.items.len()) && {
        // let matching = own.iter().zip(other.iter()).filter(|&(own, other)| *own.blocking_read() == *other.blocking_read()).count();
        // matching == own.len() && matching == other.len()
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
