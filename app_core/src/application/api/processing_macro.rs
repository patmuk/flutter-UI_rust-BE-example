use std::io;

use super::lifecycle::{Effect, Lifecycle};
use crate::{
    application::app_state::AppState,
    domain::todo_list::{TodoListModel, TodoListProcessingError},
};

use cps::cps;
// use cps::cps;
// use cps::{cps, include};
use quote::quote;
use syn::parse_macro_input;

// macro_rules! provide_api {
//      ($($file:tt),+ ) => {
//     // ($file:tt) => {
//         provide_api!(@convert
//             // $file
//             $(
//                 include!($file);
//                 // const _: &'static str = $file;
//             )+
//         );
//     };
//     (@convert $($content:tt)+) => {
//         provide_api!(@parse
//             proc_macro2::TokenStream::from($($content))+
//         )
//     };
//     (@parse $($converted:tt)+) => {
//         // provide_api!(@generate
//         provide_api!(@generate
//             quote::quote!(
//                 ($($converted))+
//             );
//         );
//         // );
//     //     quote!(
//     //     mod olala{
//     //         $(
//     //             $content
//     //         )+
//     //     }
//     // );
//     };
//     (@generate $($parsed:tt)+) => {
//         // provide_api!(
//             mod washamwa {
//             // parse_quote!(
//                 $($parsed)+
//             // );
//             }
//         // );
//     };
// }
macro_rules! provide_api {
     ($($file:tt),+ ) => {
        provide_api!(@convert
            $(
                include!($file);
            )+
            const INCLUDED: usize = 1;
        );
    };
    (@convert $($content:tt)+) => {
        provide_api!(@parse
            $(
                $content
            )+
            const CONVERTED: usize = 1;
        );
    };
    (@parse $($converted:tt)+) => {
        provide_api!(@generate
            $(
                $converted
            )+
            const PARSED: usize = 1;
        );
    };
    (@generate $($parsed:tt)+) => {
        mod washamwa {
            $(
                $parsed
            )+
            const FINAL: usize = 1;
            }
    };
}

provide_api!("../../domain/effects.rs");
// provide_api!("../../domain/todo_list.rs", "../../domain/effects.rs");

// #[cps]
// macro_rules! provide_api {
//      ($file:tt) =>
//     //  ($($file:tt),+ ) =>
//     //  let  ($($file:tt),+ ) = include!($($file)+) in
//      let $content:tt = cps::include!($file) in
//     //  let $parsed:tt = cps::quote!($file) in
//     //  let ($($content:tt)+) = cps::include!($file) in
//     //  let $file = include!($file) in

//     {
//         mod olala{
//             $(
//                 $content
//             )+
//         }
//     };

// }
// provide_api!("app_core/src/domain/effects.rs");

// #[cfg(test)]
// #[test]
// fn expand_macro() {
//     let gened = provide_api!("src/domain/todo_list.rs");
//     assert_eq!("", gened)
// }
/////////////
