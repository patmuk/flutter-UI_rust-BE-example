use super::{
    app_state_db_persister::AppStateDBPersisterError,
    app_state_file_persister::AppStateFilePersisterError,
};

#[derive(thiserror::Error, Debug)]
pub enum AppStatePersistError {
    #[error("Disk error: {0}")]
    DiskError(#[from] AppStateFilePersisterError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] AppStateDBPersisterError),

    #[error("Other persistence error: {0}")]
    Other(String),
}
