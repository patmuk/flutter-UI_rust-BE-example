use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::application::bridge::frb_generated::RustAutoOpaque;

pub(crate) trait AppState {
    fn load_or_new(app_config: &impl AppConfig) -> Self;
    fn persist(&self, path: &Path) -> Result<(), io::Error>;
    fn get_dirty_flag(&self) -> AtomicBool;
    fn mark_dirty(&self) {
        self.get_dirty_flag().store(true, Ordering::SeqCst);
    }
    // fn get_model() -> RustAutoOpaque<CqrsModel>;
    // fn get_model() -> impl CqrsModel;
}

pub(crate) trait AppConfig {}

pub(crate) trait ProcessingError: Error {}

pub(crate) trait Effect: Debug {}
pub(crate) trait CqrsModel {}
pub(crate) trait Cqrs: Debug
where
    // this constraints the CqrsModel to be wrapped in RustAutoOpaque<CqrsModel>
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<<Self as Cqrs>::Model>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    type Model: CqrsModel;
    type Effect: Effect;
    type ProcessingError: ProcessingError;
    fn process(
        self,
        model: &RustAutoOpaque<Self::Model>,
    ) -> Result<Vec<Self::Effect>, Self::ProcessingError>;
}
