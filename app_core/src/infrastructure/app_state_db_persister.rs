use log::debug;
use serde::{Deserialize, Serialize};
use std::io;

use crate::application::api::lifecycle::{
    AppConfig, AppState, AppStatePersistError, AppStatePersister, ProcessingError,
};

#[derive(Debug)]
pub struct AppStateDBPersister {
    pub(crate) url: String,
}

// handle errors as suggested by https://kazlauskas.me/entries/errors
#[derive(thiserror::Error, Debug)]
pub enum AppStateDBPersisterError {
    #[error("Cannot access the db with url: {1}")]
    IOError(#[source] io::Error, String),
    #[error("could not understand (=deserialize) the file {1}. Maybe it's content got corrupted?")]
    DeserializationError(#[source] bincode::Error, String),
    #[error("No Entry not found in db: {0}")]
    EntryNotFound(String),
}

impl AppStatePersistError for AppStateDBPersisterError {
    fn to_processing_error(&self) -> crate::application::api::lifecycle::ProcessingError {
        match self {
            Self::IOError(_err, path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_owned(),
            },
            Self::DeserializationError(_err, path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_owned(),
            },
            Self::EntryNotFound(path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_owned(),
            },
        }
    }
}

impl From<(io::Error, String)> for AppStateDBPersisterError {
    fn from((err, path): (io::Error, String)) -> Self {
        if err.kind() == io::ErrorKind::NotFound {
            AppStateDBPersisterError::EntryNotFound(path)
        } else {
            AppStateDBPersisterError::IOError(err, path)
        }
    }
}

impl From<(bincode::Error, String)> for AppStateDBPersisterError {
    fn from((err, path): (bincode::Error, String)) -> Self {
        AppStateDBPersisterError::DeserializationError(err, path)
    }
}

/// Stores the app's state in a database.
///
/// # Errors
///
/// This function will return an error if anything goes wrong
impl AppStatePersister for AppStateDBPersister {
    type Error = AppStateDBPersisterError;
    /// prepares loading and persisting the app's state
    fn new<AppConfigImpl: AppConfig>(
        _app_config: &AppConfigImpl,
    ) -> Result<AppStateDBPersister, Self::Error> {
        unimplemented!()
    }
    /// Persists the application state to storage.
    /// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
    fn persist_app_state<AS: AppState + Serialize + std::fmt::Debug>(
        &self,
        app_state: &AS,
    ) -> Result<(), Self::Error> {
        debug!(
            "persisting app state:\n  {app_state:?}\n to db {:?}",
            self.url
        );
        #[allow(unused_variables)]
        let serialized_app_state: Vec<u8> = bincode::serialize(app_state).expect(
            "bincode.serialzation itself should not result in an error, \
                unless the contract with serde is not respected!",
        );
        todo!("implement db persistance");
        // trace!("Persisted app state to db: {:?}", self.url);
        // Ok(())
    }

    // get the last persisted app state from a file, if any exists, otherwise creates a new app state
    // this function is only called once, in the initialization/app state constructor
    fn load_app_state<AC: AppConfig, AS: AppState + for<'a> Deserialize<'a>>(
        &self,
    ) -> std::result::Result<AS, Self::Error> {
        debug!("loading the app state from {:?}", self.url);
        unimplemented!("load the file from a db");
        // let app_state: Sefl::AppState = bincode::deserialize(&loaded)
        //     .map_err(|e| AppStateLoadError::DeserializationError(e, self.url))?;
        // Ok(app_state)
    }
}

#[cfg(test)]
mod tests {}
