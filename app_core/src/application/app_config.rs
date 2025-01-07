use std::path::PathBuf;

use log::{debug, trace};

use super::api::lifecycle::AppConfig;

#[derive(Debug)]
pub struct AppConfigImpl {
    /// app state storage location
    pub app_state_file_path: PathBuf,
}

impl Default for AppConfigImpl {
    fn default() -> Self {
        AppConfigImpl {
            app_state_file_path: PathBuf::from("./app_state_model.bin"),
        }
    }
}

impl AppConfig for AppConfigImpl {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(path: Option<String>) -> Self {
        match path {
            Some(path) => {
                trace!(
                "Overwriting default setup:\n  - setting the app_state_storage_path to {path:?}"
            );
                AppConfigImpl {
                    app_state_file_path: PathBuf::from(path),
                }
            }
            None => {
                debug!("Using default path in setup");
                AppConfigImpl::default()
            }
        }
    }
    fn get_app_state_url(&self) -> &PathBuf {
        &self.app_state_file_path
    }
}
