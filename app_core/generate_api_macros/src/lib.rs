extern crate proc_macro;

use log::{debug, info, trace};
use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;
use simple_logger;
use std::{fs, path::Path, str::FromStr};
use syn::{
    parse::{self, Parse, ParseStream},
    parse_file, parse_macro_input, DeriveInput, File, ItemStruct, LitStr,
};

#[proc_macro]
pub fn generate_api(file_pathes: TokenStream) -> TokenStream {
    simple_logger::init_with_level(log::Level::Trace).unwrap();
    log::info!("-------- Generating API --------");

    // Read the file locations from the input token stream
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
    info!("   for: {:?}", file_pathes);

    let content: String = file_pathes
        .into_iter()
        .map(|file_path| fs::read_to_string(file_path).unwrap())
        .collect();
    trace!("file content: \n{}", content);

    let ast = syn::parse_file(&content).unwrap();
    // println!("---------------- {} items", ast.items.len());

    let items = &ast.items;
    // println!("-------------- items: \n{:?}", items);

    let domain_model_name = items
        .iter()
        .filter_map(|item| match item {
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
                match *item_impl.self_ty.clone() {
                    syn::Type::Path(type_path) => Some(type_path.path),
                    _ => None,
                }
            }
            _ => None,
        })
        .map(|ident| ident.get_ident().unwrap().to_string())
        .collect::<String>();
    debug!("domain model name: {:?}", domain_model_name);

    let cqrs_fns = items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Impl(item_impl)
                // if item_impl.trait_.is_some()
                //     && item_impl
                //         .trait_
                //         .clone()
                //         .unwrap()
                //         .1
                //         .segments
                //         .iter()
                //         .any(|segment| segment.ident == "CqrsModel")
                 =>
            {
                Some(item_impl)
            }
            _ => None,
        })
        // .filter_map(|item| match *item.self_ty.clone() {
        //     syn::Type::Path(type_path) => Some(type_path.path),
        //     _ => None,
        // })
        // .map(|ident| ident.get_ident().unwrap().to_string())
        .collect::<Vec<_>>();

    trace!("----------- parsed items: {:?}\n", cqrs_fns);

    let expanded = quote! {
        // #ast
            fn output() {
            // let file_content = #ast;
            // let count = #count;
            // let enum_name = #enum_name;
            // println!("{}", file_content);
            println!("hello");
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
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
