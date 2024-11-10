use std::fmt::Debug;

use crate::application::api::processing::{Effect, ProcessingError};

// pub(crate) trait CqrsModel: std::marker::Sized {}
// pub(crate) trait CqrsModelLock<CqrsModel>:
//     Default + From<CqrsModel> + std::marker::Sized + Clone
// {
// }

pub trait Cqrs: Debug {
    fn process(self) -> Result<Vec<Effect>, ProcessingError>;
}
