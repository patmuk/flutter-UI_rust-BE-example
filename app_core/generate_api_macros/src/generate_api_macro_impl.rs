use log::{debug, info, trace};

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, File, Ident, Result};

pub(crate) fn generate_api_impl(file_pathes: TokenStream) -> Result<TokenStream> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    // simple_logger::init_with_level(log::Level::Trace).unwrap();
    log::info!("-------- Generating API --------");

    let ast = parse_rust_file_content(file_pathes)?;
    // .unwrap_or_else(|e| e.to_compile_error());
    //expect("File pathes should start from the project root");

    let domain_model_struct_name = get_domain_model_struct_name(&ast);
    debug!("domain model name: {:#?}", domain_model_struct_name);

    // get he processing error implementing enum
    let processing_error_enum = get_processing_error_enum_idents(&ast);

    debug!(
        "----------- processing error enum(s): {:#?}\n",
        processing_error_enum
    );

    // let cqrs_fns = ast
    //     .items
    //     .iter()
    //     // get all funtions from 'impl domain_model_struct'
    //     .filter_map(|item| match item {
    //         syn::Item::Impl(item_impl)
    //             if item_impl.trait_.is_none() && {
    //                 match *item_impl.self_ty.clone() {
    //                     syn::Type::Path(type_path) => {
    //                         *type_path.path.is_ident(domain_model_struct_name)
    //                     }
    //                     _ => false,
    //                 }
    //             } =>
    //         {
    //             Some(&item_impl.items)
    //         }
    //         _ => None,
    //     })
    //     // filter for -> Result<
    //     // .filter_map(|item_impl| match item_impl.items {
    //     //     syn::ItemImpl::Fn(fn_) => Some(fn_),
    //     //     _ => None,
    //     // })
    //     // .map(|ident| ident.get_ident().unwrap().to_string())
    //     // .collect::<String>();
    //     .collect::<Vec<_>>();

    // // debug!("----------- parsed items: {:#?}\n", cqrs_fns);

    // generate the code

    let processing_error = processing_error_enum.first().unwrap();
    let generated_code = quote! {
        // #ast
        #[derive(thiserror::Error, Debug)]
        pub enum ProcessingError {
            // #[error("Error during processing: {0}")]
            // #processing_error(#processing_error),
            #[error("Processing was fine, but state could not be persisted: {0}")]
            NotPersisted(#[source] io::Error),
        }
    };
    debug!(
        "----------- generated code: {}\n",
        prettyplease::unparse(&syn::parse2::<File>(generated_code.clone()).unwrap())
    );
    Ok(generated_code)
}

fn get_processing_error_enum_idents(ast: &File) -> Vec<String> {
    ast.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Enum(item_enum)
                if item_enum.attrs.iter().any(|attribute| {
                    attribute.path().is_ident("derive")
                        && attribute
                            .to_token_stream()
                            .to_string()
                            .contains("thiserror")
                }) =>
            {
                Some(item_enum.ident.to_string())
            }
            _ => None,
        })
        .collect::<Vec<String>>()
}

fn parse_rust_file_content(file_pathes: TokenStream) -> Result<syn::File> {
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
                        format!("Error loading the given files: {io_error}\nlooked in: {cwd:?} / \"{file_path}\""),
                    )
                }
                Err(cwd_io_error) =>                     
                        syn::Error::new(
                        Span::call_site(),
                    format!(
                        "Error reading cwd: {cwd_io_error}\nwhile loading the given files: {io_error}"
                    ),
            ),
            }
        }))
    .collect::<syn::Result<String>>()?;
    trace!("file content: \n{}", content);
    syn::parse_file(&content)
}

fn get_domain_model_struct_name(ast: &File) -> Result<String> {
    let domain_model_name = ast
        .items
        .iter()
        .filter_map(|item| match item {
            // syn::Item::Impl(item_impl)
            syn::Item::Impl(item_impl)
                if item_impl.trait_.is_some()
                    && item_impl
                        .trait_
                        .clone()
                        .unwrap()
                        .1
                        .segments
                        .iter()
                        .any(|segment| segment.ident == "CqrsModel") =>
            {
                match item_impl.self_ty.as_ref() {
                    syn::Type::Path(type_path) => Some(&type_path.path),
                    _ => None,
                }
            }
            _ => None,
        })
        .filter_map(|path| Some(path.get_ident()?.to_string()))
        .collect::<Vec<String>>();
    if domain_model_name.len() != 1 {
        return Err(syn::Error::new(
            Span::call_site(),
            "expected exactly one domain model struct",
        ));
    }
    Ok(domain_model_name[0].clone())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use quote::quote;
//     use syn;

//     #[test]
//     fn parse_fn() {
//         let source = r#"
//         fn main() {
//             let string = "line one
//             line two";
//         }
//         "#;

//         let syntax: File = syn::parse_str(source).unwrap();
//         println!("{:#?}\n", syntax);

//         println!("{}", quote!(#syntax));
//     }
// }
