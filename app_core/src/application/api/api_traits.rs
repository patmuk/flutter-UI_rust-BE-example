use log::debug;
use std::path::PathBuf;

pub trait Lifecycle {
    /// the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    /// to avoid an illegal state (app state not loaded) we do the setup and init in one go
    /// get the instance with get()
    fn new(path: Option<String>) -> &'static Self;
    fn get() -> &'static Self;
    fn app_config(&self) -> &impl AppConfig;
    fn app_state(&self) -> &impl AppState;
    /// call to initialize the app.
    /// loads the app's state, which can be io-heavy
    fn init<AC: AppConfig, AS: AppState>(app_config: &AC) -> AS {
        debug!("Initializing app with config: {:?}", app_config);
        AppState::load_or_new(app_config)
    }
    fn persist(&self) -> Result<(), std::io::Error>;
    fn shutdown(&self) -> Result<(), std::io::Error>;
}

pub trait AppConfig: Default + std::fmt::Debug {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(path: Option<String>) -> Self;
    // app state storage location
    fn app_state_file_path(&self) -> &std::path::PathBuf;
}
pub trait AppState {
    fn load_or_new<A: AppConfig>(app_config: &A) -> Self
    where
        Self: Sized;
    #[allow(clippy::ptr_arg)] // must be PathBuf; as Path isn't Size, frb can't transport it
    fn persist_to_path(&self, path: &PathBuf) -> Result<(), std::io::Error>;
    fn dirty_flag_value(&self) -> bool;
    fn mark_dirty(&self);
}
