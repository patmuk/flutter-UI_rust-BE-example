#[cfg(test)]
use generate_api_macros::generate_api;
use log::info;
include!("fixtures/good_source_file.rs");

#[test]
// mod
// fixtures::good_source_file;
// use crate::good_source_file;

fn test_macro() {
    generate_api!("app_core/generate_api_macros/tests/fixtures/good_source_file.rs");

    // assert_eq!(A::hello_macro(), "A".to_string());
}

#[test]
fn test_build() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    // simple_logger::init_with_level(log::Level::Trace).unwrap();
    info!("test llloooooogggg");
    let t = trybuild::TestCases::new();
    t.pass("tests/ui_tests.rs");
}
