use log::{info, trace};
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::Result;

pub (crate) fn read_rust_file_content(file_pathes: TokenStream) -> Result<String> {
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

    let content = file_pathes
        .into_iter()
        .map(|file_path| std::fs::read_to_string(&file_path)
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
            ),    
        )
    .collect::<syn::Result<String>>()?;
    trace!("file content: \n{}", content);
    Ok (content)
}
