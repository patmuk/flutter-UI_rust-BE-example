use log::{debug, info, trace};
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::Result;

use crate::utils::file_location_2_base_path::file_location_2_base_path;

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
pub (crate) fn read_rust_file_content(file_path: &str) -> Result<(String, String)> {

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
