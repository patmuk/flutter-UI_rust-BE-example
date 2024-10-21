extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;
use std::{fs, path::Path, str::FromStr};
use syn::{
    parse::{self, Parse, ParseStream},
    parse_file, parse_macro_input, DeriveInput, File, ItemStruct, LitStr,
};

#[proc_macro]
pub fn generate_api(file_pathes: TokenStream) -> TokenStream {
    // Read the file locations from the input token stream
    // println!("-------------- Raw input: {:?}", file_pathes);

    let content: String = file_pathes
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
        .map(|file_path| fs::read_to_string(file_path).unwrap())
        .collect();
    println!("-------------- file content: \n{}", content);

    let ast = syn::parse_file(&content).unwrap();
    // let ast: TokenStream = syn::parse_str(&content).unwrap();
    // println!("---------------- {} items", ast.items.len());

    let items = &ast.items;
    // println!("-------------- items: \n{:?}", items);

    // // get cqrs functions
    // // let cqrs_functions =
    // items
    //     .iter()
    //     // .for_each(|item| println!("----------- item: {:?}\n", item));
    //     .for_each(|item| println!("----------- item: {:?}\n", item));
    // // .filter_map(|item| match item {
    // //     syn::Item::Const(item_const) => todo!(),
    // //     syn::Item::Enum(item_enum) => todo!(),
    // //     syn::Item::ExternCrate(item_extern_crate) => todo!(),
    // //     syn::Item::Fn(item_fn) => todo!(),
    // //     syn::Item::ForeignMod(item_foreign_mod) => todo!(),
    // //     syn::Item::Impl(item_impl) => todo!(),
    // //     syn::Item::Macro(item_macro) => todo!(),
    // //     syn::Item::Mod(item_mod) => todo!(),
    // //     syn::Item::Static(item_static) => todo!(),
    // //     syn::Item::Struct(item_struct) => todo!(),
    // //     syn::Item::Trait(item_trait) => todo!(),
    // //     syn::Item::TraitAlias(item_trait_alias) => todo!(),
    // //     syn::Item::Type(item_type) => todo!(),
    // //     syn::Item::Union(item_union) => todo!(),
    // //     syn::Item::Use(item_use) => todo!(),
    // //     syn::Item::Verbatim(token_stream) => todo!(),
    // //     _ => todo!(),
    // // })
    // // .collect();

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
                Some(item_impl)
            }
            _ => None,
        })
        .collect::<Vec<&syn::ItemImpl>>();
    // .for_each(|item| {
    //     // println!("----------- parsed item: {:?}\n", item);
    //     // println!("----------- parsed item name: {:?}\n", item.ident);
    //     // println!("----------- parsed item attributes: {:?}\n", item.attrs);
    //     // println!("----------- parsed item items: {:?}\n", item.items);
    //     println!("----------- parsed item trait: {:?}\n", item.self_ty);
    // });

    let domain_model_name: String = domain_model_name
        .into_iter()
        .filter_map(|item| match *item.self_ty.clone() {
            syn::Type::Path(type_path) => Some(type_path.path),
            _ => None,
        })
        .map(|ident| ident.get_ident().unwrap().to_string())
        .collect();

    println!(
        "----------- parsed item domain name: {:?}\n",
        domain_model_name
    );

    let expanded = quote! {
        // #ast
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
