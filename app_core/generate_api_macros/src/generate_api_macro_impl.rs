use log::debug;

use crate::utils::read_rust_files::{read_rust_file_content, tokens_2_file_locations};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_str, File, Result};

pub(crate) fn generate_api_impl(file_pathes: TokenStream) -> Result<TokenStream> {
    simple_logger::init_with_level(log::Level::Debug).expect("faild to init logger");

    log::info!("-------- Generating API --------");

    let file_locations = tokens_2_file_locations(file_pathes)?;
    let (base_path, file_content) = read_rust_file_content(&file_locations[0])?;

    // TODO implement for more than one file
    let ast = syn::parse_file(&file_content)?;

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

    // TODO support multiplr inputs
    // let processing_error = format_ident!("{}", processing_error_enum.first().unwrap());
    let processing_error = format_ident!(
        "{}",
        processing_error_enum
            .first()
            .expect("error formating the ident")
    );

    let use_statement_processing_error_enum = format!(
        "use {}::{};",
        base_path,
        processing_error_enum
            .first()
            .expect("error formating the ident")
    );
    // parse with syn
    let use_statements = parse_str::<File>(&use_statement_processing_error_enum)
        .expect("error parsing use statement");

    let use_statement = use_statements
        .items
        .first()
        .expect("first item was not the use statement");

    let generated_code = quote! {
        #use_statement

        #[derive(thiserror::Error, Debug)]
        pub enum ProcessingError {
            #[error("Error during processing: {0}")]
            #processing_error(#processing_error),
            #[error("Processing was fine, but state could not be persisted: {0}")]
            NotPersisted(#[source] std::io::Error),
        }
    };
    debug!(
        "----------- generated code:\n\n{}\n",
        prettyplease::unparse(&syn::parse2::<File>(generated_code.clone()).unwrap())
    );
    Ok(generated_code)
}

/// searches the error enum(s) by derive thiserror only!
// / TODO search for the word "Error" in the name?
/// TODO search for one error enum, panic if more are present?
fn get_processing_error_enum_idents(ast: &File) -> Vec<String> {
    println!("----------- get processing error enum idents:");
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
                        .expect("Should have gotten a trait")
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

#[cfg(test)]
mod tests {
    use crate::{
        generate_api_macro_impl::get_processing_error_enum_idents,
        utils::read_rust_files::read_rust_file_content,
    };

    thread_local! {
        static AST: syn::File = syn::parse_file(
            &read_rust_file_content("tests/fixtures/src/good_source_file/mod.rs")
            .unwrap()
            .1,
        ).unwrap();
    }

    #[test]
    fn generate_error_enum_test() {
        let result = AST.with(get_processing_error_enum_idents);
        assert_eq!(
            "MyGoodProcessingError".to_string(),
            *result.first().unwrap()
        );
    }
}
