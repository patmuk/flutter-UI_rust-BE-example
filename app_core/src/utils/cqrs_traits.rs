use flutter_rust_bridge::frb;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::application::bridge::frb_generated::{
    MoiArc, MoiArcValue, RustAutoOpaque, RustAutoOpaqueMoi,
};

pub trait AppState {
    fn load_or_new(app_config: &impl AppConfig) -> Self;
    fn persist(&self, path: &Path) -> Result<(), io::Error>;
    fn get_dirty_flag(&self) -> AtomicBool;
    fn mark_dirty(&self) {
        self.get_dirty_flag().store(true, Ordering::SeqCst);
    }
    // fn get_model() -> RustAutoOpaque<CqrsModel>;
    // fn get_model() -> impl CqrsModel;
}

pub trait AppConfig {}

pub trait ProcessingError: Error {}

pub trait Effect: Debug {}
pub trait CqrsModel {}
pub trait Cqrs: Debug
where
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<<Self as Cqrs>::Model>:
        crate::application::bridge::frb_generated::MoiArcValue,
{
    // }

    // pub trait Cqrs: Debug {
    // pub trait Cqrs<E: Effect, PE: ProcessingError>: Debug {
    // pub trait Cqrs<M: CqrsModel, E: Effect, PE: ProcessingError>: Debug {
    type Model: CqrsModel;
    type Effect: Effect;
    type ProcessingError: ProcessingError;
    fn process(
        self,
        model: &RustAutoOpaque<Self::Model>,
    ) -> Result<Vec<Self::Effect>, Self::ProcessingError>;
    // fn process(self, model: &Self::Model) -> Result<Vec<impl Effect>, impl ProcessingError>;
}
// pub trait Cqrs: Debug {
//     type Model;
//     fn process(self, model: &Self::Model) -> Result<Vec<impl Effect>, impl ProcessingError>;
//     // fn process(self, model: &Self::Model) -> Result<Vec<impl Effect>, impl ProcessingError>;
// }

// pub trait Cqrs: Debug {
//     fn process(
//         self,
//         model: &impl crate::application::bridge::frb_generated::MoiArcValue,
//     ) -> Result<Vec<impl Effect>, impl ProcessingError>;
// }
// pub trait Cqrs: Debug {
//     fn process(
//         self,
//         model: &flutter_rust_bridge::for_generated::RustAutoOpaqueBase<
//             crate::domain::todo_list::TodoListModel,
//             crate::application::bridge::frb_generated::MoiArc<
//                 flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
//                     crate::domain::todo_list::TodoListModel,
//                 >,
//             >,
//         >,
//     ) -> Result<Vec<impl Effect>, impl ProcessingError>;
// }

// #[cfg(test)]
// mod tests {
//     use super::Effect;

//     #[derive(Debug)]
//     struct DerivenEffect {
//         name: String,
//     }

//     impl Effect for DerivenEffect {
//         fn get_name(&self) -> String {
//             self.name.to_string()
//         }
//     }

//     #[test]
//     fn test_deriving() {
//         let deriven = DerivenEffect {
//             name: "Ho!".to_string(),
//         };
//         assert_eq!(
//             deriven.say_my_name(),
//             "I am Ho!: DerivenEffect { name: \"Ho!\" }"
//         );
//     }
// }
