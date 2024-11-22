pub mod application;
pub mod domain;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("faild to init logger");
}
