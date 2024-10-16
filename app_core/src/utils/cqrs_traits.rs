use std::fmt::Debug;

use crate::application::bridge::frb_generated::RustAutoOpaque;
use crate::domain::app_state::AppState;
use crate::domain::effects::Effect;
use crate::domain::processing_errors::ProcessingError;

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
    fn get_model(app_state: &AppState) -> &RustAutoOpaque<Self>;
}
pub trait Cqrs: Debug {
    fn process(self, app_state: &AppState) -> Result<Vec<Effect>, ProcessingError>;
    fn is_command(&self) -> bool;
    fn is_query(&self) -> bool {
        !self.is_command()
    }
}
