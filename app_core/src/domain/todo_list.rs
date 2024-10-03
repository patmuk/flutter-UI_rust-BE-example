use crate::application::bridge::frb_generated::RustAutoOpaque;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
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
    pub items: RustAutoOpaque<Vec<String>>,
}

// impl TodoListModel {
//     pub fn get_items(&self) -> &Vec<String> {
//         &*self.items.blocking_read()
//     }
// }

impl Default for TodoListModel {
    fn default() -> Self {
        TodoListModel {
            items: RustAutoOpaque::new(Vec::default()),
        }
    }
}

impl Serialize for TodoListModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.items.blocking_read().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for TodoListModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::deserialize(deserializer).map(|items| TodoListModel {
            items: RustAutoOpaque::new(items),
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
    RenderTodoList(RustAutoOpaque<Vec<String>>),
}

pub(crate) fn process_command_todo_list(
    command: Command,
    model: &mut TodoListModel,
) -> Vec<Effect> {
    match command {
        Command::AddTodo(todo) => {
            model.items.blocking_write().push(todo);
            // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
            vec![Effect::RenderTodoList(model.items.clone())]
        }
        Command::RemoveTodo(todo_pos) => {
            model
                .items
                .blocking_write()
                .remove((todo_pos - 1).try_into().unwrap());
            vec![Effect::RenderTodoList(model.items.clone())]
        }
        Command::CleanList => {
            model.items.blocking_write().clear();
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

    fn assert_eq_effects(actual_effect: &Effect, expected_effect: &Effect) {
        assert!(
            matches!((actual_effect, expected_effect), (Effect::RenderTodoList(actual), Effect::RenderTodoList(expected)) if {
                *actual.blocking_read() == *expected.blocking_read()
            })
        )
    }

    #[test]
    fn add_todo() {
        //        let todo_list = AppTester::<TodoList, _>::default();
        let mut actual_model = TodoListModel::default();

        // Call 'add'
        let actual_effects =
            process_command_todo_list(Command::AddTodo("test the list".into()), &mut actual_model);

        let expected_model = TodoListModel {
            items: RustAutoOpaque::new(vec![String::from("test the list")]),
        };
        let expected_effect = Effect::RenderTodoList(expected_model.items.clone());
        let actual_effect = &actual_effects[0];
        assert_eq_effects(actual_effect, &expected_effect);
        assert_eq!(
            *actual_model.items.blocking_read(),
            *expected_model.items.blocking_read()
        );
    }

    #[test]
    fn remove_todo() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut actual_model = TodoListModel {
            items: RustAutoOpaque::new(vec!["remove me".into()]),
        };

        // Call 'add'
        let actual_effects = process_command_todo_list(Command::RemoveTodo(1), &mut actual_model);

        let expected_model = TodoListModel {
            items: RustAutoOpaque::new(vec![]),
        };
        let expected_effect = [Effect::RenderTodoList(expected_model.items.clone())];
        assert_eq_effects(&actual_effects[0], &expected_effect[0]);
        assert_eq!(
            *actual_model.items.blocking_read(),
            *expected_model.items.blocking_read()
        );
    }

    #[test]
    fn clean_list() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut actual_model = TodoListModel::default();
        actual_model.items.blocking_write().push("remove me".into());
        actual_model.items.blocking_write().push("clean me".into());

        // Call 'add'
        let actual_effects = process_command_todo_list(Command::CleanList, &mut actual_model);

        let expected_model = TodoListModel {
            items: RustAutoOpaque::new(vec![]),
        };
        let expected_effects = [Effect::RenderTodoList(expected_model.items.clone())];
        assert_eq_effects(&actual_effects[0], &expected_effects[0]);
        assert_eq!(
            *actual_model.items.blocking_read(),
            *expected_model.items.blocking_read()
        );
    }
}
