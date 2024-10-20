extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::{fs, path::Path};
use syn::{parse_file, parse_macro_input, DeriveInput, File};

#[proc_macro]
pub fn generate_api(input: TokenStream) -> TokenStream {
    // Read the file locations from the input token stream
    let file_path_literal = input.to_string().replace("\"", "");
    // let file_path_literal = input.to_string();
    println!("-------------- file name: {}", file_path_literal);
    let file_content = fs::read_to_string(file_path_literal).unwrap();
    // let file_content = file_path_literal;
    println!("-------------- file content: {}", file_content);
    // let ast = syn::parse_file(&file_content).unwrap();
    let ast = syn::parse_str::<File>(&file_content).unwrap();
    println!("---------------- {} items", ast.items.len());

    let enum_name = ast.items.first().unwrap();
    let count = ast.items.len();

    let expanded = quote! {
        fn output() {
            // let file_content = #ast;
            let count = #count;
            let enum_name = #enum_name;
            println!("{}", file_content);
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

// #[proc_macro]
// pub fn generate_api(input: TokenStream) -> TokenStream {
//     // let filename = input.to_string();
//     let name = input.to_string();
//     let input = syn::parse_file(&name).unwrap();
//     // Parse the input tokens into a syntax tree
//     // let input = parse_macro_input!(input as DeriveInput);
//     // let input = parse_macro_input!(input as File);
//     // let name = input.ident.to_string();
//     // let name = input.items.first().unwrap();
//     let expanded = quote! {
//         fn output() -> print!(name)
//     };

//     // Hand the output tokens back to the compiler
//     TokenStream::from(expanded)
// }

// #[proc_macro]
// pub fn generate_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let signature = syn::parse_macro_input!(input as Signature);
//     let function = signature.function;
//     let arguments = signature.arguments;
//     let return_t = signature.return_t;

//     if let 1 = arguments.len() {
//         let arg = &arguments[0];
//         let tokens = quote! {
//             fn #function(arg: #arg) -> #return_t {
//                 let ret: #return_t = arg * 2;
//                 println!("input {} * 2 = {}", arg, ret);
//                 ret
//             }
//         };
//         tokens.into()
//     } else {
//         panic!("Invalid input");
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
