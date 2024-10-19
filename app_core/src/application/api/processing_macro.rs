use std::io;

use super::lifecycle::{Effect, Lifecycle};
use crate::{
    application::app_state::AppState,
    domain::todo_list::{TodoListModel, TodoListProcessingError},
};

// use cps::cps;
// use cps::{cps, include};
use syn::parse_macro_input;

macro_rules! provide_api {
     ($($file:tt),+ ) => {
    // ($file:tt) => {
        provide_api!(@parse
            // $file
            $(
                const _: &'static str = $file;
            )+
        );
    };
    (@parse $($content:tt)+) => {
        // provide_api!(@read_file $content);
        pub mod olala{
            $(
                $content
            )+
        }
    };

     // ($($file:tt),+ ) => {
       //     // fn output() {
       //         provide_api!(@parse
       //         $(
       //         // let file = include!(concat!("../../", $file));
       //         include!($file);
       //             // println!("{}", $file);
       //         )*
       //         );
       //     };
       // (@parse $content:tt) => {
       //     pub mod olala{
       //         $content
       //     }
       // };
}

// provide_api!("../../domain/effects.rs");
provide_api!("../../domain/todo_list.rs", "../../domain/effects.rs");

// #[cps] // Add this macro to unlock additional syntax
// macro_rules! provide_api {
//     ($file:literal) =>
//     //let $content:tt =
//     {
//         include!(concat!("../../", $file)) in
//             stringify!(
//                 $content
//             )
//     };
// }

// // #[cps]
// // macro_rules! provide_api {
// //     ( ) =>
// // in
// //  {
// //     };
// // }

// provide_api!("domain/todo_list.rs");

// #[cfg(test)]
// #[test]
// fn expand_macro() {
//     let gened = provide_api!("src/domain/todo_list.rs");
//     assert_eq!("", gened)
// }
/////////////
