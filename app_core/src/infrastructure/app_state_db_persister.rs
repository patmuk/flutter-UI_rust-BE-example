use log::trace;
use std::io;
use std::path::PathBuf;

use crate::application::{
    api::lifecycle::AppStatePersister, app_config::AppConfigImpl, app_state::AppStateImpl,
};

use super::app_state_persistance_error::AppStatePersistError;

pub(crate) struct UnititializedAppStateDBPersister {}

#[derive(Debug)]
pub(crate) struct AppStateDBPersister {
    pub(crate) url: String,
}

// handle errors as suggested by https://kazlauskas.me/entries/errors
#[derive(thiserror::Error, Debug)]
pub(crate) enum AppStateDBPersisterError {
    #[error("Cannot access the db with url: {1}")]
    IOError(#[source] io::Error, PathBuf),
    #[error("could not understand (=deserialize) the file {1}. Maybe it's content got corrupted?")]
    DeserializationError(#[source] bincode::Error, PathBuf),
    #[error("No Entry not found in db: {0}")]
    EntryNotFound(PathBuf),
}

/// Stores the app's state in a database.
///
/// # Errors
///
/// This function will return an error if anything goes wrong
impl AppStatePersister for AppStateDBPersister {
    type AppConfig = AppConfigImpl;
    type AppState = AppStateImpl;
    /// prepares loading and persisting the app's state
    fn new(_app_config: &Self::AppConfig) -> Result<AppStateDBPersister, AppStatePersistError> {
        unimplemented!()
    }
    /// Persists the application state to storage.
    /// Ensures that the `AppState` is stored in a durable way, regardless of the underlying mechanism.
    fn persist_app_state(&self, app_state: &Self::AppState) -> Result<(), AppStatePersistError> {
        trace!(
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
    fn load_app_state(&self) -> Result<Self::AppState, AppStatePersistError> {
        trace!("loading the app state from {:?}", self.url);
        unimplemented!("load the file from a db");
        // let app_state: Sefl::AppState = bincode::deserialize(&loaded)
        //     .map_err(|e| AppStateLoadError::DeserializationError(e, self.url))?;
        // Ok(app_state)
    }
}

#[cfg(test)]
mod tests {}
