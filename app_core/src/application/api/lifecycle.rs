use crate::domain::app_state::{self, AppState};

use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

use flutter_rust_bridge::frb;
// implement logging, as shown in https://github.com/fzyzcjy/flutter_rust_bridge/issues/252
use log::debug;
use parking_lot::RwLock;

use log::{debug, error};

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
pub static INSTANCE: OnceLock<Lifecycle> = OnceLock::new();

// only pub functions to call
/// instanciate the lazy static app state -
/// call if you want to control when the app state is initialized,
/// which might take time (due to IO when loading the last saved state)
/// otherwise it is called automatically when the lazy reference is accessed the first time
pub fn init<'a>() -> AppStateReference<'a> {
    if (IS_INITIALIZED.load(Ordering::Relaxed)) {
        panic!("already initialized!");
    }
    ensure_logger_is_set_up();
    // let _ = &*API;
    // let _ = &*APP_STATE;
    // get_state();
    let mut app_config = APP_CONFIG.get();
    // let app_config = APP_CONFIG.get().get_or_insert(&AppConfig::default());
    let app_state = AppState::load_or_new(app_config.get_or_insert(&AppConfig::default()));
    APP_STATE.set(RwLock::new(app_state));
    let app_state_ref = AppStateReference {
        lock: APP_STATE.get().expect("app_state has been set"),
    };
    IS_INITIALIZED.store(true, Ordering::Relaxed);
    app_state_ref
}

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// call to overwrite default values.
/// Doesn't trigger initialization.
// TODO implement after v2 upgrade
// pub fn setup(app_config: AppConfig) -> Result<(), io::Error> {
// pub fn setup(app_config: AppConfig) {
pub fn setup(path: Option<String>) {
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

// app state storage location
#[derive(Debug)]
pub struct Lifecycle {
    app_state_lock: AppStateLock,
}

#[derive(Debug)]
#[frb(opaque)]
struct AppStateLock {
    lock: RwLock<(AppState)>,
}

// impl AppStateLock {
impl Lifecycle {
    pub fn get_lock(&self) -> &RwLock<AppState> {
        &self.app_state_lock.lock
    }
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[frb(non_opaque)]
pub struct AppConfigReference<'a> {
    app_config: &'a AppConfig,
}

pub fn get_app_config_ref<'a>() -> AppConfigReference<'a> {
    AppConfigReference {
        app_config: APP_CONFIG.get().expect("App config not set up yet"),
    }
}

// TODO implement Error handling!
// pub fn persist_app_state() -> Result<(), std::io::Error> {
pub fn persist_app_state(app_state: &AppState) {
    let app_config = APP_CONFIG
        .get()
        .expect("AppConfig must be set, error in this lib's logic flow!");
    app_state.persist(&app_config.app_state_file_path).unwrap();
}

// pub fn shutdown() -> Result<(), std::io::Error> {
/// Blocks, if RwLock<AppState> is in write use
/// TODO implent timeout and throw an error?
pub fn shutdown() -> Result<(), io::Error> {
    debug!("shutting down the app");
    if let Some(app_state) = APP_STATE.get() {
        app_state.read().expect("no error").persist(
            &APP_CONFIG
                .get_or_init(AppConfig::default)
                .app_state_file_path,
        )
    } else {
        // if the app state has not been set (e.g. init() not called),
        // no need to persist!
        Ok(())
    }
    pub fn get() -> &'static Lifecycle {
        &Self::init()
    }

// initializes the app_state only at first call
// The app state is behind a mutex to avoid data conditions, and static, to be globally available to all threads
// This is needed, as a static mut cannot be modified by save code, the mutex makes this possible
// pub(crate) static APP_STATE: LazyLock<RwLock<AppState>> = LazyLock::new(|| {
//     RwLock::new(AppState::load_or_new(
//         APP_CONFIG.get().expect("setup() has bee called before"),
//     ))
// });
#[frb(non_opaque)]
pub static APP_STATE: OnceLock<RwLock<AppState>> = OnceLock::new();

// pub fn init_app_state_lock() {
//     // let app_state = APP_STATE.get().expect("setup() has bee called before");
//     // let app_state = APP_STATE;
//     APP_STATE_LOCK
//         .set(AppStateLock { lock: APP_STATE })
//         .expect("setup() has bee called before");
// }

// static APP_STATE_LOCK: OnceLock<AppStateLock> = OnceLock::new();

/// transfer object
#[derive(Debug)]
#[frb(non_opaque)]
pub struct AppStateReference<'a> {
    #[frb(non_opaque)]
    pub lock: &'a RwLock<AppState>,
}

pub fn get_app_state_ref<'a>() -> AppStateReference<'a> {
    AppStateReference {
        lock: APP_STATE.get().expect("App state not set up yet"),
    }
}
// impl AppStateLock {
// pub fn load_or_new(app_config: AppConfig) -> AppStateLock {
//     *APP_STATE_LOCK.get_or_init(|| AppStateLock {
//         lock: RwLock::new(AppState::load_or_new(&app_config)),
//     })
// }

// }

// pub fn get_state() -> &'static RwLock<AppState> {
//     //&RwLockReadGuard<AppState> {
//     match APP_STATE.get() {
//         Some(app_state) => {
//             debug!("App State already exists, returning it");
//             app_state
//         }
//         None => {
//             APP_STATE
//                 .set(RwLock::new(AppState::load_or_new(
//                     APP_CONFIG.get_or_init(AppConfig::default),
//                 )))
//                 .expect("APP_STATE hasn't been set before");
//             &APP_STATE.get().expect("ÄPP_STATE just set")
//         }
//     }
// }

    // pub fn get_app_state(&self) -> Arc<AppState> {
    //     // Arc::clone(&self.app_state)
    // }
    // pub fn read_app_state(&self) -> &AppState {
    //     &self.app_state.read()
    // }

// static APP_STATE: OnceCell<RwLock<AppState>> = OnceCell::new();
