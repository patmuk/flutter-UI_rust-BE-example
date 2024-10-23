/// Since this is a fixture to test the proc macro,
/// we don't import all dependencies, which are probably ven (crate) private.
///
/// Instead, we include! the needed files and comment out dependencies
/// not needed for the proc macro tests.
///
///
// use crate::{
//     application::{app_state::AppState, bridge::frb_generated::RustAutoOpaque},
//     utils::cqrs_traits::CqrsModel,
// };
// use flutter_rust_bridge::frb;
// use serde::{Deserialize, Serialize};

include!("../mocks/app_state_mock.rs");
include!("../mocks/rust_auto_opaque.rs");

// #[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[derive(Debug, Default, PartialEq)]
// #[frb(opaque)]
pub struct MyGoodDomainModel {
    items: Vec<DomainItem>,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
#[derive(Debug, PartialEq)]
struct DomainItem {
    text: String,
}
pub enum MyGoodDomainModelEffect {
    RenderItems(RustAutoOpaque<MyGoodDomainModel>),
}

impl MyGoodDomainModel {
    pub fn get_items_as_string(&self) -> Vec<String> {
        self.items.iter().map(|item| item.text.clone()).collect()
    }

    pub(crate) fn add_item(
        app_state: &AppState,
        item: String,
    ) -> Result<Vec<MyGoodDomainModelEffect>, MyGoodProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        model_lock.blocking_write().items.push(item { text: todo });
        app_state.mark_dirty();
        // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
        Ok(vec![MyGoodDomainModelEffect::RenderTodoList(
            model_lock.clone(),
        )])
    }
    pub(crate) fn remove_item(
        app_state: &AppState,
        todo_pos: usize,
    ) -> Result<Vec<MyGoodDomainModelEffect>, MyGoodProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        let items = &mut model_lock.blocking_write().items;
        if todo_pos > items.len() {
            Err(MyGoodProcessingError::ItemDoesNotExist(todo_pos))
        } else {
            items.remove(todo_pos - 1);
            app_state.mark_dirty();
            Ok(vec![MyGoodDomainModelEffect::RenderTodoList(
                model_lock.clone(),
            )])
        }
    }
    pub(crate) fn clean_list(
        app_state: &AppState,
    ) -> Result<Vec<MyGoodDomainModelEffect>, MyGoodProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        model_lock.blocking_write().items.clear();
        app_state.mark_dirty();
        Ok(vec![MyGoodDomainModelEffect::RenderTodoList(
            model_lock.clone(),
        )])
    }
    pub(crate) fn get_all_items(
        app_state: &AppState,
    ) -> Result<Vec<MyGoodDomainModelEffect>, MyGoodProcessingError> {
        let model_lock = TodoListModel::get_model_lock(app_state);
        Ok(vec![MyGoodDomainModelEffect::RenderTodoList(
            model_lock.clone(),
        )])
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MyGoodProcessingError {
    #[error("The todo at index {0} does not exist!")]
    ItemDoesNotExist(usize),
}

impl CqrsModel for MyGoodDomainModel {
    /// bootstrap the model from the app's state
    fn get_model_lock(app_state: &AppState) -> &RustAutoOpaque<Self> {
        &app_state.model
    }
}

// only for tests, as the danger for a deadlock is too big
#[cfg(test)]
impl PartialEq for MyGoodDomainModelEffect {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (MyGoodDomainModelEffect::RenderTodoList(own), MyGoodDomainModelEffect::RenderTodoList(other)) if {
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
    use crate::application::app_state::AppState;

    use super::*;

    #[test]
    fn add_todo() {
        let expected_model = RustAutoOpaque::new(MyGoodDomainModel {
            items: vec![TodoItem {
                text: String::from("test the list"),
            }],
        });
        let expected_MyGoodDomainModelEffects = vec![MyGoodDomainModelEffect::RenderTodoList(
            expected_model.clone(),
        )];

        let actual_model = RustAutoOpaque::new(MyGoodDomainModel::default());
        let app_state = AppState::from_model(&actual_model);
        let actual_MyGoodDomainModelEffects =
            MyGoodDomainModel::add_todo(&app_state, "test the list".into()).unwrap();

        assert_eq!(
            actual_MyGoodDomainModelEffects,
            expected_MyGoodDomainModelEffects
        );
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }

    #[test]
    fn remove_todo() {
        let expected_model = RustAutoOpaque::new(MyGoodDomainModel { items: vec![] });
        let expected_MyGoodDomainModelEffects = vec![MyGoodDomainModelEffect::RenderTodoList(
            expected_model.clone(),
        )];

        let actual_model = RustAutoOpaque::new(MyGoodDomainModel {
            items: vec![TodoItem {
                text: "remove me".into(),
            }],
        });
        let app_state = AppState::from_model(&actual_model);
        let actual_MyGoodDomainModelEffects =
            MyGoodDomainModel::remove_todo(&app_state, 1).unwrap();

        assert_eq!(
            actual_MyGoodDomainModelEffects,
            expected_MyGoodDomainModelEffects
        );
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
        assert_eq!(
            MyGoodDomainModel::remove_todo(&app_state, 1),
            Err(MyGoodProcessingError::ItemDoesNotExist(1))
        );
    }

    #[test]
    fn clean_list() {
        let expected_model = RustAutoOpaque::new(MyGoodDomainModel { items: vec![] });
        let expected_MyGoodDomainModelEffects = vec![MyGoodDomainModelEffect::RenderTodoList(
            expected_model.clone(),
        )];

        let actual_model = RustAutoOpaque::new(MyGoodDomainModel::default());
        actual_model.blocking_write().items.push(TodoItem {
            text: "remove me".into(),
        });
        actual_model.blocking_write().items.push(TodoItem {
            text: "clean me".into(),
        });
        let app_state = AppState::from_model(&actual_model);

        let actual_MyGoodDomainModelEffects = MyGoodDomainModel::clean_list(&app_state).unwrap();

        assert_eq!(
            actual_MyGoodDomainModelEffects,
            expected_MyGoodDomainModelEffects
        );
        assert_eq!(
            *actual_model.blocking_read(),
            *expected_model.blocking_read()
        );
    }
}
