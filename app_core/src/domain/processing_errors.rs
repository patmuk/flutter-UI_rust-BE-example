/// All Errors that can occur during processing
/// One enum, so that
/// - handling for a consumer is easier (one match)
///   (a wrapper enum with Defref impl might work as well)
/// - errors are reused among domain implementations (keeping the number low)
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ProcessingError {
    #[error("The todo at index {0} does not exist!")]
    TodosDoesNotExist(usize),
}
