use std::io;

use super::lifecycle::{Effect, Lifecycle};
use crate::{
    application::app_state::AppState,
    domain::todo_list::{TodoListModel, TodoListProcessingError},
};
use generate_api_macros::generate_api;

//generate_api!();

// #[macro_export]
// macro_rules! parse_and_generate_api {
//     //  ($file:tt) =>
//     ($($file:tt),+ ) => {
//         // include!($($file)+);
//         generate_api!(
//             $(
//                 include!(
//                     $file
//                 )
//             )+;
//         );
//     };

//  ($($file:tt),+ ) => {
//     parse_and_generate_api!(@generate
//         $(
//             include!($file);
//         )+
//         const INCLUDED: usize = 1;
//     );
// };
// (@generate $($content:tt)+) => {
//     generate_api!(
//     // print!(
//     $(
//             $content
//         )+
//         const CONVERTED: usize = 1;
//     );
// };
// }

// generate_api!("../../domain/effects.rs");
#[cps::cps]
macro_rules! gen_api {
    ($source:literal) =>
    let $($file_content:tt)* = cps::include!($source) in
    {
        generate_api!($($file_content)*)
    }
}

// generate_api!("app_core/src/domain/effects.rs");
gen_api!("app_core/src/domain/effects.rs");

// gen_api!("../../domain/todo_list.rs", "../../domain/effects.rs");
// generate_api!("../../domain/todo_list.rs", "../../domain/effects.rs");
// parse_and_generate_api!("../../domain/effects.rs");
//provide_api!("../../domain/effects.rs");

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
