use std::{
    cell::OnceCell,
    sync::{LazyLock, OnceLock},
};

#[cfg(test)]
use crate::read_rust_file_content;
const TEST_FILE: String = read_rust_file_content("fixtures/src/good_source_file.rs");
const AST: LazyLock<syn::File> = LazyLock::new(|| syn::parse_file(&TEST_FILE).unwrap());

#[test]
fn generate_error_enum_test() {
    let result = get_processing_error_enum_idents(AST);
    assert_eq!("bla", result);
}
