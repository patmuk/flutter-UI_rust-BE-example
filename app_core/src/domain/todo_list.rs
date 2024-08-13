use serde::{Deserialize, Serialize};

use crate::application::api::lifecycle::API;

// TODO remove clone and handle references!
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TodoListModel {
    pub items: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    AddTodo(String),
    RemoveTodo(u32),
    CleanList,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Query {
    GetModel,
    AllTodos,
}

// requests to the shell, aka Capabilities aka Effects
#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    Render(TodoListModel),
    RenderTodoList(Vec<String>),
}

pub(crate) fn process_command_todo_list(
    command: Command,
    model: &mut TodoListModel,
) -> Vec<Effect> {
    match command {
        Command::AddTodo(todo) => {
            model.items.push(todo);
            // TODO use reference, not clone
            vec![Effect::RenderTodoList(model.items.clone())]
        }
        Command::RemoveTodo(todo_pos) => {
            model.items.remove((todo_pos - 1).try_into().unwrap());
            // TODO use reference, not clone
            vec![Effect::Render(model.clone())]
        }
        Command::CleanList => {
            model.items = vec![];
            // TODO use reference, not clone
            vec![Effect::Render(model.clone())]
        }
    }
}

pub(crate) fn process_query_todo_list(query: Query) -> Vec<Effect> {
    match query {
        // TODO use reference, not clone
        Query::GetModel => vec![Effect::Render(API.read().model.clone())],
        Query::AllTodos => vec![Effect::RenderTodoList(API.read().model.items.clone())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crux_core::{render::RenderOperation, testing::AppTester};

    #[test]
    fn add_todo() {
        //        let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();

        // Call 'add'
        let effects =
            process_command_todo_list(Command::AddTodo("test the list".into()), &mut model);

        let expected_model = TodoListModel {
            items: vec![String::from("test the list")],
        };
        let expected_effect = Effect::Render(expected_model);
        let actual_effect = &effects[0];
        assert_eq!(actual_effect, &expected_effect);
    }

    #[test]
    fn remove_todo() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel {
            items: vec!["remove me".into()],
        };

        // Call 'add'
        let effects = process_command_todo_list(Command::RemoveTodo(1), &mut model);

        let expected_model = TodoListModel { items: vec![] };
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(expected_model);
        assert_eq!(actual_effect, &expected_effect);
    }
    #[test]
    fn clean_list() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();
        model.items.push("remove me".into());
        model.items.push("clean me".into());

        // Call 'add'
        let effects = process_command_todo_list(Command::CleanList, &mut model);

        let expected_model = TodoListModel { items: vec![] };
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(expected_model);
        assert_eq!(actual_effect, &expected_effect);
    }
}
