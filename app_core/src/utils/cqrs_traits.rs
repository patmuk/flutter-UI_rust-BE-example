pub(crate) trait CqrsModel: std::marker::Sized {}
pub(crate) trait CqrsModelLock: Default + From<Self::Model> + std::marker::Sized {
    type Lock;
    type Model: CqrsModel;
    fn get<'a>(&'a self) -> &'a Self::Lock;
    fn clone(&self) -> Self::Lock;
}
pub trait Cqrs {
    type Effect;
    type ProcessingError;
    fn process(self) -> Result<Vec<Self::Effect>, Self::ProcessingError>;
}
