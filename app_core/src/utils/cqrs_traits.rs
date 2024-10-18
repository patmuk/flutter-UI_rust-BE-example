use std::fmt::Debug;
use std::sync::Arc;

use crate::application::api::lifecycle::{WrappedCqrs, Wrapping};
use crate::application::app_state::AppState;
use crate::application::bridge::frb_generated::RustAutoOpaque;
use crate::application::processing_errors::ProcessingError;
use crate::domain::effects::Effect;

// pub(crate) trait AppState {
//     fn load_or_new(app_config: &impl AppConfig) -> Self;
//     fn persist(&self, path: &Path) -> Result<(), io::Error>;
//     fn get_dirty_flag(&self) -> AtomicBool;
//     fn mark_dirty(&self) {
//         self.get_dirty_flag().store(true, Ordering::SeqCst);
//     }
//     // fn get_model() -> RustAutoOpaque<CqrsModel>;
//     // fn get_model() -> impl CqrsModel;
// }

// pub(crate) trait AppConfig {}

pub(crate) trait CqrsModel: std::marker::Sized
where
    // this constraints the CqrsModel to be wrapped in RustAutoOpaque<CqrsModel>
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Self>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    fn get_model_lock(app_state: &AppState) -> &RustAutoOpaque<Self>;
}
pub trait Cqrs: Wrapping + Debug {
    fn is_command(&self) -> bool;
    fn is_query(&self) -> bool {
        !self.is_command()
    }
}
