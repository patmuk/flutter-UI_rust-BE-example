use flutter_rust_bridge::frb;

use super::bridge::frb_generated::RustAutoOpaque;

pub struct AppState {
    pub items: Vec<RustAutoOpaque<Heavy>>,
}
impl AppState {
    pub fn get_first_item(&self) -> RustAutoOpaque<Heavy> {
        self.items[0].clone() // only clones the Arc so it is lightweight
    }
}

#[frb(opaque)]
#[derive(Debug, Clone)] // TODO remove clone
pub struct Heavy {
    pub heavy: u32,
}

fn add_item(appstate: &mut AppState) {
    appstate
        .items
        .push(RustAutoOpaque::new(Heavy { heavy: 10 }));
}
async fn display_first_item(appstate: &AppState) {
    let fist_item = appstate.get_first_item();
    let heavy = fist_item.read().await;
    print!("The first item is {:?}", heavy);
}
