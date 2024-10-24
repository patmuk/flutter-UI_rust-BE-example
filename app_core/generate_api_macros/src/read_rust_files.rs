use log::{debug, info, trace};
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::Result;

/// extracts file locations from a TokenStream
pub (crate) fn tokens_2_file_locations(file_pathes: TokenStream) -> Result<Vec<String>> {
    let file_pathes = file_pathes
        .into_iter()
        .filter_map(|token| match token {
            TokenTree::Literal(literal) => Some(literal),
            _ => None,
        })
        .map(|literal| {
            // if to_string() breaks, parse it with https://github.com/LukasKalbertodt/litrs/
            let cleaned = literal.to_string();
            cleaned[1..cleaned.len() - 1].to_string()
        })
        .collect::<Vec<String>>();
    info!("Parsing content of: {:#?}", file_pathes);
    Ok(file_pathes)
}

/// reads a rust file and returns (path, content)
pub (crate) fn read_rust_file_content(file_path: String) -> Result<(String, String)> {

    let path = file_location_2_base_path(&file_path);
    
    debug!("base path is: {:#?}", path);

    let content = std::fs::read_to_string(&file_path)
        .map_err(|io_error| {
            let current_dir = std::env::current_dir();
            match current_dir {
                Ok(cwd) => {
                    syn::Error::new(
                        Span::call_site(),
                        format!("Error loading the given files: {io_error}\nlooked in: {cwd:?} / \"{file_path}\"\nFile pathes need start from the project root."),
                    )
                }
                Err(cwd_io_error) =>                     
                        syn::Error::new(
                        Span::call_site(),
                    format!(
                        "Error reading cwd: {cwd_io_error}\nwhile loading the given files: {io_error}\nFile pathes need start from the project root."),
                    ),
                }
            }    
        )?;
    trace!("file content: \n{}", content);
    Ok ((path, content))
}

fn file_location_2_base_path(file_path: &str) -> String {
    let mut path_split = file_path.split('/')
    .skip_while(|element| *element != "src");
    path_split.next().expect("file path needs to contain 'src/'");
    let base_path = path_split.collect::<Vec<&str>>().join("::");
    format!("crate::{}", &base_path[..base_path.len()-3])
}


#[cfg(test)]
mod tests {
use crate::read_rust_files::file_location_2_base_path;

#[test]
#[should_panic(expected = "file path needs to contain 'src/'")]
fn test_file_location_2_base_path_no_src() {
    let input = String::from("main.rs");
    file_location_2_base_path(&input);
}
#[test]
fn test_file_location_2_base_path_zero_levels() {
    let input = String::from("src/main.rs");
    assert_eq!("crate::main".to_string(), file_location_2_base_path(&input));
}
#[test]
fn test_file_location_2_base_path_one_level() {
    let input = String::from("src/domain/model.rs");
    assert_eq!("crate::domain::model".to_string(), file_location_2_base_path(&input));
}
#[test]
fn test_file_location_2_base_path_two_levels() {
    let input = String::from("src/domain/model/item.rs");
    assert_eq!("crate::domain::model::item".to_string(), file_location_2_base_path(&input));
}
#[test]
fn test_file_location_2_base_path_multiple_levels() {
    let input = String::from("src/domain/model/items/entity.rs");
    assert_eq!("crate::domain::model::items::entity".to_string(), file_location_2_base_path(&input));
}
}