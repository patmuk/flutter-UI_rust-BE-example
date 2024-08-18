use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
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
pub enum Effect<'a> {
    // parameters need to be owned - as they are taken from
    // the APP_STATE. This is guarded by a mutex to handle concurrent access.
    // Either we pass the whole mutex or we clone selected subfields
    Render(&'a TodoListModel),
    RenderTodoList(&'a Vec<String>),
}

pub(crate) fn process_command_todo_list(
    command: Command,
    model: &'static mut TodoListModel,
) -> Vec<Effect> {
    match command {
        Command::AddTodo(todo) => {
            model.items.push(todo);
            vec![Effect::RenderTodoList(&model.items)]
        }
        Command::RemoveTodo(todo_pos) => {
            model.items.remove((todo_pos - 1).try_into().unwrap());
            vec![Effect::RenderTodoList(&model.items)]
        }
        Command::CleanList => {
            model.items = vec![];
            vec![Effect::RenderTodoList(&model.items)]
        }
    }
}

pub(crate) fn process_query_todo_list(query: Query, model: &TodoListModel) -> Vec<Effect> {
    match query {
        // need to use clone here, as the RWLock is mutex guarding the value.
        // So either pass the RWLock or clone the model
        Query::GetModel => vec![Effect::Render(&model)],
        Query::AllTodos => vec![Effect::RenderTodoList(&model.items)],
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
        let expected_effect = Effect::Render(&expected_model);
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
        let expected_effect = Effect::Render(&expected_model);
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
        let expected_effect = Effect::Render(&expected_model);
        assert_eq!(actual_effect, &expected_effect);
    }
}
