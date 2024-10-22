use generate_api_macros::generate_api;

fn main() {
    generate_api!("../../../../app_core/generate_api_macros/tests/fixtures/good_source_file.rs");
}
