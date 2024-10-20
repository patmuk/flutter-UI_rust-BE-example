extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;
use std::{fs, path::Path, str::FromStr};
use syn::{
    parse::{self, Parse, ParseStream},
    parse_file, parse_macro_input, DeriveInput, File, LitStr,
};

// struct GenerateApiInput {
//     file_pathes: Vec<String>,
// }

// impl Parse for GenerateApiInput {
//     fn parse(input: ParseStream) -> Result<Self> {
//         input
//     }
// }

#[proc_macro]
pub fn generate_api(file_pathes: TokenStream) -> TokenStream {
    // Read the file locations from the input token stream
    // println!("-------------- Raw input: {:?}", file_pathes);

    let file_pathes_filtered: Vec<String> = file_pathes
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
        .collect();

    println!("-------------- Cleaned: {:?}", file_pathes_filtered);

    // load and compine the files's content

    let content = file_pathes_filtered
        .into_iter()
        .map(|file_path| fs::read_to_string(file_path).unwrap())
        .collect::<String>();
    println!("-------------- file content: \n{}", content);

    let ast = syn::parse_file(&content).unwrap();
    // println!("---------------- {} items", ast.items.len());

    let expanded = quote! {
        #ast
        //     fn output() {
    //         // let file_content = #ast;
    //         let count = #count;
    //         let enum_name = #enum_name;
    //         println!("{}", file_content);
    //     }
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
