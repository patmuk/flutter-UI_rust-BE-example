use crate::application::api::api_traits::AppState;
use crate::application::bridge::frb_generated::RustAutoOpaque;

pub(crate) trait CqrsModel: std::marker::Sized
where
    // this constraints the CqrsModel to be wrapped in RustAutoOpaque<CqrsModel>
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Self>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    fn get_model_lock<AS: AppState>(app_state: &AS) -> &RustAutoOpaque<Self>;
}
