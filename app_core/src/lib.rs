pub mod api;
pub mod app_state;
mod bridge;
mod todo_list;

use log::{debug, LevelFilter};
use once_cell::sync::OnceCell;
use oslog::OsLogger;

static LOGGER: OnceCell<()> = OnceCell::new();

/// Sets up the global logger that is used in this crate
/// Only first call actually sets up the logger, subsequent calls are no-ops
fn ensure_logger_is_set_up() {
    LOGGER.get_or_init(|| {
        // initialize logger
        OsLogger::new("com.example.todo_app")
            .level_filter(LevelFilter::Trace)
            .category_level_filter("Settings", LevelFilter::Trace)
            .init()
            .unwrap();
        debug!("logger is set up!");
    });
}
