use generate_api_macros::generate_api;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("Current directory is: {}", cwd.display());

    // generate_api!("app_core/generate_api_macros/tests/fixtures/good_source_file.rs");
    // generate_api!("/Users/patmuk/code/own/sherry/examples/flutter-rust-bridge_crux_style/app_core/generate_api_macros/tests/fixtures/good_source_file.rs");
    // let file_location = concat!(
    //     env!("CARGO_MANIFEST_DIR"),
    //     "/tests/fixtures/good_source_file.rs"
    // );
    // let file_location = format!("{:?}/../tests/fixtures/good_source_file.rs", cwd);
    // generate_api!(file_location);
    // generate_api!(
    // );
    generate_api!("../../../../app_core/generate_api_macros/tests/fixtures/good_source_file.rs");
    // generate_api!("tests/fixtures/good_source_file.rs");
    // generate_api!("./tests/fixtures/good_source_file.rs");
    // generate_api!("../fixtures/good_source_file.rs");
}
