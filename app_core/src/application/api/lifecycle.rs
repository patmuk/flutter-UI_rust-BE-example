use crate::application::app_config::AppConfig;
use crate::application::app_state::AppState;
use crate::ensure_logger_is_set_up;

use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

use flutter_rust_bridge::frb;
// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::debug;
use parking_lot::RwLock;

// static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
pub static INSTANCE: OnceLock<Lifecycle> = OnceLock::new();

// pub fn get_app_state() -> Arc<AppState> {
//     match INSTANCE.get() {
//         Some(instance) => instance.get_app_state(),
//         None => panic!("The Lifecycle has not been initialized"),
//     }
// }

pub struct Lifecycle {
    pub app_state_lock: AppStateLock,
}

#[frb(opaque)]
pub struct AppStateLock {
    pub lock: RwLock<(AppState)>,
}

pub fn setup(path: Option<String>) {
    ensure_logger_is_set_up();

    let app_config = match path {
        Some(path) => {
            debug!(
                "Overwriting default setup:\n  - setting the app_state_storage_path to {path:?}"
            );
            AppConfig {
                app_state_file_path: PathBuf::from(path),
            }
        }
        None => {
            debug!("Using default path in setup");
            AppConfig::default()
        }
    };
    APP_CONFIG.set(app_config);
}

impl Lifecycle {
    /// call to overwrite default values.
    /// Doesn't trigger initialization.
    // TODO implement after v2 upgrade
    // pub fn setup(app_config: AppConfig) -> Result<(), io::Error> {

    /// call to initialize the app.
    /// loads the app's state, which can be io-heavy
    pub fn init() -> &'static Self {
        // if !IS_INITIALIZED.load(Ordering::Relaxed) {
        match INSTANCE.get() {
            Some(instance) => instance,
            None => {
                ensure_logger_is_set_up();
                let app_config = match APP_CONFIG.get() {
                    Some(app_config) => app_config,
                    None => &AppConfig::default(),
                };

                debug!("Initializing app with config: {:?}", app_config);
                let app_state = AppState::load_or_new(app_config);
                // IS_INITIALIZED.store(true, Ordering::Relaxed);
                let lifecycle = Lifecycle {
                    app_state_lock: AppStateLock {
                        lock: RwLock::new(app_state),
                    },
                };
                INSTANCE.set(lifecycle);
                Lifecycle::get()
            }
        }
    }
    pub fn get() -> &'static Lifecycle {
        &Self::init()
    }

    // pub fn read(self) -> &'static Self {
    //     if !IS_INITIALIZED.load(Ordering::Relaxed) {
    //         &Lifecycle::init()
    //     } else {
    //         &self
    //     }
    // }

    // pub fn get_app_state(&self) -> Arc<AppState> {
    //     // Arc::clone(&self.app_state)
    // }
    // pub fn read_app_state(&self) -> &AppState {
    //     &self.app_state.read()
    // }

    // pub fn mut_borrow_app_state<'a>(self) -> &'a mut AppState {
    //     &mut app_state
    // }

    // TODO implement Error handling!
    // pub fn persist_app_state(app_state: &AppState) {
    pub fn persist_app_state(&self) -> Result<(), std::io::Error> {
        let app_config = APP_CONFIG
            .get()
            .expect("AppConfig must be set, error in this lib's logic flow!");
        self.app_state_lock
            .lock
            .read()
            .persist(&app_config.app_state_file_path)
    }

    // pub fn shutdown() -> Result<(), std::io::Error> {
    /// Blocks, if RwLock<AppState> is in write use
    /// TODO implent timeout and throw an error?
    pub fn shutdown(&self) -> Result<(), io::Error> {
        debug!("shutting down the app");
        // if INSTANCE.get().is_some() {
        // if IS_INITIALIZED.load(Ordering::Relaxed) {
        self.app_state_lock.lock.read().persist(
            &APP_CONFIG
                .get()
                .expect("Has been initialized.")
                .app_state_file_path,
        )
        // } else {
        //     Ok(())
        // }
        // if let Some(app_state) = APP_STATE.get() {
        //     app_state.read().expect("no error").persist(
        //         &APP_CONFIG
        //             .get_or_init(AppConfig::default)
        //             .app_state_file_path,
        //     )
        // } else {
        //     // if the app state has not been set (e.g. init() not called),
        //     // no need to persist!
        //     Ok(())
        // }
    }
}
