use std::path::Path;

use log::debug;

pub trait Lifecycle {
    type AC: AppConfig;
    type AS: AppState;

    /// the app config is to be set only once, and read afterwards. If mutation is needed wrapp it into a lock for concurrent write access
    /// to avoid an illegal state (app state not loaded) we do the setup and init in one go
    /// get the instance with get()
    fn new(path: Option<String>) -> &'static Self;
    // fn new(path: Option<String>) -> &'static Self;
    // fn new<AC: AppConfig, AS: AppState>(path: Option<String>) -> &'static Lifecycle<AC, AS>;

    fn get() -> &'static Self;
    fn app_config<'a>(&'a self) -> &'a Self::AC;

    /// call to initialize the app.
    /// loads the app's state, which can be io-heavy
    fn init(app_config: &Self::AC) -> Self::AS {
        debug!("Initializing app with config: {:?}", app_config);
        AppState::load_or_new(app_config)
    }
    fn persist(&self) -> Result<(), std::io::Error>;

    fn shutdown(&self) -> Result<(), std::io::Error>;
}

// app state storage location
pub trait AppConfig: Default + std::fmt::Debug {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    fn new(path: Option<String>) -> Self;
    fn app_state_file_path(&self) -> &std::path::Path;
}

pub(crate) trait AppState {
    fn load_or_new<A: AppConfig>(app_config: &A) -> Self
    where
        Self: Sized;
    fn persist_to_path(&self, path: &Path) -> Result<(), std::io::Error>;
    // fn get_dirty_flag(&self) -> std::sync::atomic::AtomicBool;
    fn mark_dirty(&self);
    // fn get_model() -> RustAutoOpaque<CqrsModel>;
    // fn get_model() -> impl CqrsModel;
}
