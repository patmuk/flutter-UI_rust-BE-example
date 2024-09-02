use flutter_rust_bridge::frb;

use crate::ensure_logger_is_set_up;

use super::bridge::frb_generated::RustAutoOpaque;

#[derive(Default)]
// #[frb(non_opaque)]
#[frb(opaque)]
pub struct AppState {
    pub items: Vec<RustAutoOpaque<Heavy>>,
}
impl AppState {
    pub fn new() -> Self {
        ensure_logger_is_set_up();
        AppState::default()
    }
    pub fn get_first_item(&self) -> RustAutoOpaque<Heavy> {
        self.items[0].clone() // only clones the Arc so it is lightweight
    }
}
pub fn add_item(app_state: &mut AppState) {
    app_state
        .items
        .push(RustAutoOpaque::new(Heavy { heavy: 10 }));
}
pub async fn display_first_item(appstate: &AppState) {
    let fist_item = appstate.get_first_item();
    let heavy = fist_item.read().await;
    print!("The first item is {:?}", heavy);
}

#[frb(opaque)]
#[derive(Debug, Clone)] // TODO remove clone
pub struct Heavy {
    pub heavy: u32,
}
