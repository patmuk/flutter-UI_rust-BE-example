// use crate::{
//     application::{app_state::AppState, bridge::frb_generated::RustAutoOpaque},
//     utils::cqrs_traits::CqrsModel,
// };
// use flutter_rust_bridge::frb;
// use serde::{Deserialize, Serialize};

// include!("../mocks/app_state_mock.rs");
// include!("../mocks/rust_auto_opaque_mock.rs");
// include!("../mocks/cqrs_traits_mock.rs");
use crate::mocks::app_state_mock::AppState;
use crate::mocks::cqrs_traits_mock::CqrsModel;
use crate::mocks::rust_auto_opaque_mock::RustAutoOpaque;

// const app_state_const: AppState = AppState {};
// const MODEL_LOCK: RustAutoOpaque<MyGoodDomainModel> = RustAutoOpaque {
//     model: MyGoodDomainModel { items: vec![] },
// };
// std::sync::RwLock::new(MyGoodDomainModel { items: vec![] });
// #[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[derive(Clone, Debug, Default, PartialEq)]
// #[frb(opaque)]
pub struct MyGoodDomainModel {
    items: Vec<DomainItem>,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
#[derive(Clone, Debug, PartialEq)]
struct DomainItem {
    text: String,
}
pub enum MyGoodDomainModelEffect {
    RenderItems(RustAutoOpaque<MyGoodDomainModel>),
}

#[allow(dead_code)]
impl MyGoodDomainModel {
    pub fn get_items_as_string(&self) -> Vec<String> {
        self.items.iter().map(|item| item.text.clone()).collect()
    }

    pub(crate) fn add_item(
        app_state: &AppState,
        item: String,
    ) -> Result<Vec<MyGoodDomainModelEffect>, MyGoodProcessingError> {
        let model_lock = Self::get_model_lock(app_state);
        model_lock
            .blocking_write()
            .items
            .push(DomainItem { text: item });
        app_state.mark_dirty();
        // this clone is cheap, as it is on ARC (RustAutoOpaque>T> = Arc<RwMutex<T>>)
        Ok(vec![MyGoodDomainModelEffect::RenderItems(
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
            Ok(vec![MyGoodDomainModelEffect::RenderItems(
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
        Ok(vec![MyGoodDomainModelEffect::RenderItems(
            model_lock.clone(),
        )])
    }
    pub(crate) fn get_all_items(
        app_state: &AppState,
    ) -> Result<Vec<MyGoodDomainModelEffect>, MyGoodProcessingError> {
        let model_lock = MyGoodDomainModel::get_model_lock(app_state);
        Ok(vec![MyGoodDomainModelEffect::RenderItems(
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
    fn get_model_lock(_app_state: &AppState) -> &RustAutoOpaque<Self> {
        let neverused = Box::new(RustAutoOpaque {
            model: MyGoodDomainModel::default(),
        });
        Box::leak(neverused)
    }
}
