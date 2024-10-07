use crate::application::bridge::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
use serde::{ser::SerializeSeq, Deserialize, Serialize};

#[derive(Debug, Default)]
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
    pub items: Vec<RustAutoOpaque<TodoItem>>,
}

// only for tests, as the danger for a deadlock is too big
#[cfg(test)]
impl PartialEq for TodoListModel {
    fn eq(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }
        let matching = self
            .items
            .iter()
            .zip(other.items.iter())
            .filter(|&(own, other)| *own.blocking_read() == *other.blocking_read())
            .count();
        matching == self.items.len() && matching == other.items.len()
    }
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

impl Serialize for TodoListModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // get lock and serialize all items
        // deadlock is avoided, as a lock as to be required by the process methods
        let mut ser = serializer.serialize_seq(Some(self.items.len()))?;
        for item in &self.items {
            ser.serialize_element(&*item.blocking_read())?
        }
        ser.end()
    }
}
impl<'de> Deserialize<'de> for TodoListModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // deserialize all items
        let items = Vec::deserialize(deserializer)?;
        // for item in &mut items {
        Ok(TodoListModel {
            items: items.into_iter().map(RustAutoOpaque::new).collect(),
        })
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
    RenderTodoList(Vec<RustAutoOpaque<TodoItem>>),
}

// only for tests, as the danger for a deadlock is too big
#[cfg(test)]
impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (Effect::RenderTodoList(own), Effect::RenderTodoList(other)) if {
            (own.len() == other.len()) && {
                // be aware of a potential deadlock here!
                // (lock on own, waiting for other and in another thread vice-versa!)
        let matching = own.iter().zip(other.iter()).filter(|&(own, other)| *own.blocking_read() == *other.blocking_read()).count();
        matching == own.len() && matching == other.len()
        }})
    }
}

pub(crate) fn process_command_todo_list(
    command: Command,
    model: &mut TodoListModel,
) -> Vec<Effect> {
    match command {
        Command::AddTodo(todo) => {
            // model.items.blocking_write().push(todo);
            model
                .items
                .push(RustAutoOpaque::new(TodoItem { text: todo }));
            // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
            vec![Effect::RenderTodoList(model.items.clone())]
        }
        Command::RemoveTodo(todo_pos) => {
            model.items.remove((todo_pos - 1).try_into().unwrap());
            vec![Effect::RenderTodoList(model.items.clone())]
        }
        Command::CleanList => {
            model.items.clear();
            vec![Effect::RenderTodoList(model.items.clone())]
        }
    }
}

pub(crate) fn process_query_todo_list(query: Query, model: &TodoListModel) -> Vec<Effect> {
    match query {
        // the clone here is cheap, as it clones `RustAutoOpaque<T> = Arc<RwMutex<T>>`
        Query::AllTodos => vec![Effect::RenderTodoList(model.items.clone())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_todo() {
        let mut actual_model = TodoListModel::default();

        let actual_effects =
            process_command_todo_list(Command::AddTodo("test the list".into()), &mut actual_model);

        let expected_model = TodoListModel {
            items: vec![RustAutoOpaque::new(TodoItem {
                text: String::from("test the list"),
            })],
        };
        let expected_effect = Effect::RenderTodoList(expected_model.items.clone());
        let actual_effect = &actual_effects[0];
        assert_eq!(actual_effect, &expected_effect);
        assert_eq!(actual_model, expected_model);
    }

    #[test]
    fn remove_todo() {
        let mut actual_model = TodoListModel {
            items: vec![RustAutoOpaque::new(TodoItem {
                text: "remove me".into(),
            })],
        };

        let actual_effects = process_command_todo_list(Command::RemoveTodo(1), &mut actual_model);

        let expected_model = TodoListModel { items: vec![] };
        let expected_effect = Effect::RenderTodoList(expected_model.items.clone());
        assert_eq!(&actual_effects[0], &expected_effect);
        assert_eq!(actual_model, expected_model);
    }

    #[test]
    fn clean_list() {
        let mut actual_model = TodoListModel::default();
        actual_model.items.push(RustAutoOpaque::new(TodoItem {
            text: "remove me".into(),
        }));
        actual_model.items.push(RustAutoOpaque::new(TodoItem {
            text: "clean me".into(),
        }));

        let actual_effects = process_command_todo_list(Command::CleanList, &mut actual_model);

        let expected_model = TodoListModel { items: vec![] };
        let expected_effects = vec![Effect::RenderTodoList(expected_model.items.clone())];
        assert_eq!(&actual_effects, &expected_effects);
        assert_eq!(actual_model, expected_model);
    }
}
