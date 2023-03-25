// use crux_core::{render::Render, App, Capability};
// use crux_macros::Effect;
use serde::{Deserialize, Serialize};
// #[derive(Default)]
// pub struct TodoList;

#[derive(Default)]
pub(crate) struct TodoListModel {
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

// #[derive(Effect)]
// #[effect(app = "TodoList")]
// pub struct Capabilities {
//     render: Render<Event>,
// }

// requests to the shell, aka Capabilities aka Effects
#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    Render(ViewModel),
}

// impl App for TodoList {
// impl TodoList {
// type Event = Event;
// type Model = TodoListModel;
// type ViewModel = ViewModel;
// type Capabilities = Vec<Request>;

// fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
// pub fn update(&self, event: Event, model: &mut TodoListModel) -> Vec<Request> {
pub(crate) fn update(event: Event, model: &mut TodoListModel) -> Vec<Effect> {
    match event {
        Event::AddTodo(todo) => model.items.push(todo),
        Event::RemoveTodo(todo_pos) => {
            model.items.remove(todo_pos - 1);
        }
        Event::CleanList => model.items = vec![],
    }
    // caps.render.render();
    vec![Effect::Render(view(model))]
}

// pub fn view(&self, model: &TodoListModel) -> ViewModel {
pub(crate) fn view(model: &TodoListModel) -> ViewModel {
    let count = model.items.len();
    ViewModel {
        items: model.items.clone(),
        count,
    }
}
// }
// temporarily adapted for use with flutter-rust-bridge
// impl TodoList {
//     fn new() -> Self {
//         Self {}
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    // use crux_core::{render::RenderOperation, testing::AppTester};

    #[test]
    fn add_todo() {
        //        let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();

        // Call 'add'
        let effects = /*todo_list.*/update(Event::AddTodo("test the list".into()), &mut model);

        // Check update asked us to `Render`
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(view(&model));
        assert_eq!(actual_effect, &expected_effect);

        // Make sure the view matches our expectations
        let actual_view = /*&todo_list.*/view(&model);
        let expected_view = ViewModel {
            items: vec!["test the list".into()],
            count: 1,
        };
        assert_eq!(actual_view, expected_view);
    }
    #[test]
    fn remove_todo() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel {
            items: vec!["remove me".into()],
        };

        // Call 'add'
        let effects = /*todo_list.*/update(Event::RemoveTodo(1), &mut model);

        // Check update asked us to `Render`
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(view(&model));
        assert_eq!(actual_effect, &expected_effect);

        // Make sure the view matches our expectations
        let actual_view = /*&todo_list.*/view(&model);
        let expected_view = ViewModel {
            items: vec![],
            count: 0,
        };
        assert_eq!(actual_view, expected_view);
    }
    #[test]
    fn clean_list() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();
        model.items.push("remove me".into());
        model.items.push("clean me".into());

        // Call 'add'
        let effects = /*todo_list.*/update(Event::CleanList, &mut model);

        // Check update asked us to `Render`
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(view(&model));
        assert_eq!(actual_effect, &expected_effect);

        // Make sure the view matches our expectations
        let actual_view = /*&todo_list.*/view(&model);
        let expected_view = ViewModel {
            items: vec![],
            count: 0,
        };
        assert_eq!(actual_view, expected_view);
    }
}
