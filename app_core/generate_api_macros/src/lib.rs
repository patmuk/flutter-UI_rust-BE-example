extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::{fs, path::Path, str::FromStr};
use syn::{parse_file, parse_macro_input, DeriveInput, File};

#[proc_macro]
pub fn generate_api(input: TokenStream) -> TokenStream {
    // Read the file locations from the input token stream
    let file_path_literal = input.to_string().replace("\"", "");
    println!("-------------- file name: {}", file_path_literal);
    let file_content = fs::read_to_string(file_path_literal).unwrap();
    println!("-------------- file content: \n{}", file_content);
    let ast = syn::parse_file(&file_content).unwrap();
    println!("---------------- {} items", ast.items.len());

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
