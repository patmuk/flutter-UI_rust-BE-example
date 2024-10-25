use crate::mocks::app_state_mock::AppState;
use crate::mocks::rust_auto_opaque_mock::RustAutoOpaque;

pub(crate) trait CqrsModel: std::marker::Sized {
    fn get_model_lock(app_state: &AppState) -> &RustAutoOpaque<Self>;
}
