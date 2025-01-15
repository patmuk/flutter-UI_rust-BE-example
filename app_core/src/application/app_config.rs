use log::debug;

use super::api::lifecycle::AppConfig;

#[derive(Debug)]
pub struct AppConfigImpl {
    /// app state storage location
    pub app_state_url: String,
}

impl Default for AppConfigImpl {
    fn default() -> Self {
        AppConfigImpl {
            app_state_url: "./app_state_model.bin".to_string(),
        }
    }
}

impl AppConfig for AppConfigImpl {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(url: Option<String>) -> Self {
        match url {
            Some(url) => {
                debug!(
                    "Overwriting default setup:\n  - setting the app_state_storage_url to {url:?}"
                );
                AppConfigImpl { app_state_url: url }
            }
            None => {
                debug!("Using default path in setup");
                AppConfigImpl::default()
            }
        }
    }
    fn borrow_app_state_url(&self) -> &str {
        &self.app_state_url
    }
}
