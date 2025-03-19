use log::debug;
use serde::{Deserialize, Serialize};
use std::io;

use crate::application::api::lifecycle::{
    AppConfig, AppState, AppStatePersistError, AppStatePersister, ProcessingError,
};

pub struct AppStateDBPersister {
    pub(crate) url: String,
    bincode_config: bincode::config::Configuration,
}

// handle errors as suggested by https://kazlauskas.me/entries/errors
#[derive(thiserror::Error, Debug)]
pub enum AppStateDBPersisterError {
    #[error("Cannot access the db with url: {1}")]
    IOError(#[source] io::Error, String),
    #[error("could not persist the state into file {1}. Bincode-Error: {0}")]
    SerializationError(#[source] bincode::error::EncodeError, String),
    #[error(
        "could not understand (=deserialize) the file {1}. Maybe it's content got corrupted? Bincode-Error: {0}"
    )]
    DeserializationError(#[source] bincode::error::DecodeError, String),
    #[error("No Entry not found in db: {0}")]
    EntryNotFound(String),
}

impl AppStatePersistError for AppStateDBPersisterError {
    fn to_processing_error(&self) -> crate::application::api::lifecycle::ProcessingError {
        match self {
            AppStateDBPersisterError::IOError(_err, path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_owned(),
            },
            AppStateDBPersisterError::DeserializationError(_err, path) => {
                ProcessingError::NotPersisted {
                    error: self.to_string(),
                    url: path.to_owned(),
                }
            }
            AppStateDBPersisterError::EntryNotFound(path) => ProcessingError::NotPersisted {
                error: self.to_string(),
                url: path.to_owned(),
            },
            AppStateDBPersisterError::SerializationError(_, path) => {
                ProcessingError::NotPersisted {
                    error: self.to_string(),
                    url: path.to_owned(),
                }
            }
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

impl From<(bincode::error::DecodeError, String)> for AppStateDBPersisterError {
    fn from((err, path): (bincode::error::DecodeError, String)) -> Self {
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
        app_config: &AppConfigImpl,
    ) -> Result<AppStateDBPersister, Self::Error> {
        // if this config ever depends on a user's preference we add it to the AppConfig
        let bincode_config = bincode::config::standard();
        // create the directories, but no need to write the file, as there is only the default content
        // remove the last part, as this is the file
        let url = app_config.borrow_app_state_url();
        Ok(AppStateDBPersister {
            url: url.to_string(),
            bincode_config,
        })
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
        let serialized_app_state: Vec<u8> =
            bincode::serde::encode_to_vec(app_state, self.bincode_config).expect(
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
