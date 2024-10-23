use proc_macro::TokenStream;

mod generate_api_macro_impl;
mod read_rust_files;

#[proc_macro]
pub fn generate_api(file_pathes: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file_pathes = proc_macro2::TokenStream::from(file_pathes);
    TokenStream::from(
        generate_api_macro_impl::generate_api_impl(file_pathes)
            .unwrap_or_else(|e| e.to_compile_error()),
    )
}

#[cfg(test)]
use log::info;

#[test]
fn ui() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    // simple_logger::init_with_level(log::Level::Trace).unwrap();
    info!("test llloooooogggg");
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
}
