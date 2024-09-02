use flutter_rust_bridge::frb;

use super::bridge::frb_generated::RustAutoOpaque;

#[derive(Default)]
#[frb(opaque)]
pub struct AppState {
    pub items: Vec<RustAutoOpaque<Heavy>>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            items: vec![RustAutoOpaque::new(Heavy { heavy: 42 })],
        }
    }
    pub fn get_first_item(&self) -> Option<RustAutoOpaque<Heavy>> {
        self.items.first().cloned()
        // .clone() // only clones the Arc so it is lightweight
    }
    pub fn get_last_item(&self) -> Option<RustAutoOpaque<Heavy>> {
        self.items.last().cloned()
        // .expect("there should be at least one item")
        // .clone() // only clones the Arc so it is lightweight
    }
    pub fn add_item(&mut self, item: u32) {
        self.items.push(RustAutoOpaque::new(Heavy { heavy: item }));
    }
    pub fn remove_item(&mut self, index: u32) {
        self.items.remove(index as usize);
    }

    pub async fn display_first_item(&self) {
        let fist_item = self.get_first_item();
        match fist_item {
            Some(first) => print!("The first item is {:?}", first.read().await.heavy),
            None => println!("There is no item"),
        }
    }
}

#[frb(opaque)]
#[derive(Debug)] // TODO remove clone
pub struct Heavy {
    pub heavy: u32,
}

impl Clone for Heavy {
    fn clone(&self) -> Self {
        panic!("I should not be called");
        // Self { heavy: self.heavy.clone() }
    }
}
