use std::fmt::Debug;

pub(crate) trait CqrsModel: std::marker::Sized {}
pub(crate) trait CqrsModelLock<CqrsModel>:
    Default + From<CqrsModel> + std::marker::Sized + Clone
{
}

pub trait Cqrs: Debug {
    type Effect;
    type ProcessingError: Debug;
    fn process(self) -> Result<Vec<Self::Effect>, Self::ProcessingError>;
}
