use crux_core::{render::Render, App};
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct TodoList;

#[derive(Default)]
pub struct TodoListModel {
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ViewModel {
    pub items: Vec<String>,
    pub count: usize,
}
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Event {
    AddTodo(String),
    RemoveTodo(usize),
    CleanList,
}

#[derive(Effect)]
#[effect(app = "TodoList")]
pub struct Capabilities {
    render: Render<Event>,
}

impl App for TodoList {
    type Event = Event;

    type Model = TodoListModel;

    type ViewModel = ViewModel;

    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::AddTodo(todo) => model.items.push(todo),
            Event::RemoveTodo(todo_pos) => {
                model.items.remove(todo_pos - 1);
            }
            Event::CleanList => model.items = vec![],
        }
        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        let count = model.items.len();
        ViewModel {
            items: model.items.clone(),
            count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crux_core::{render::RenderOperation, testing::AppTester};

    #[test]
    fn add_todo() {
        let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();

        // Call 'add'
        let update = todo_list.update(Event::AddTodo("test the list".into()), &mut model);

        // Check update asked us to `Render`
        let actual_effect = &update.effects[0];
        let expected_effect = &Effect::Render(RenderOperation);
        assert_eq!(actual_effect, expected_effect);

        // Make sure the view matches our expectations
        let actual_view = &todo_list.view(&model);
        let expected_view = ViewModel {
            items: vec!["test the list".into()],
            count: 1,
        };
        assert_eq!(*actual_view, expected_view);
    }
    #[test]
    fn remove_todo() {
        let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();
        model.items = vec!["remove me".into()];

        // Call 'add'
        let update = todo_list.update(Event::RemoveTodo(1), &mut model);

        // Check update asked us to `Render`
        let actual_effect = &update.effects[0];
        let expected_effect = &Effect::Render(RenderOperation);
        assert_eq!(actual_effect, expected_effect);

        // Make sure the view matches our expectations
        let actual_view = &todo_list.view(&model);
        let expected_view = ViewModel {
            items: vec![],
            count: 0,
        };
        assert_eq!(*actual_view, expected_view);
    }
    #[test]
    fn clean_list() {
        let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();
        model.items.push("remove me".into());
        model.items.push("clean me".into());

        // Call 'add'
        let update = todo_list.update(Event::CleanList, &mut model);

        // Check update asked us to `Render`
        let actual_effect = &update.effects[0];
        let expected_effect = &Effect::Render(RenderOperation);
        assert_eq!(actual_effect, expected_effect);

        // Make sure the view matches our expectations
        let actual_view = &todo_list.view(&model);
        let expected_view = ViewModel {
            items: vec![],
            count: 0,
        };
        assert_eq!(*actual_view, expected_view);
    }
}
