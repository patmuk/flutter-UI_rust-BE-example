use std::path::PathBuf;

// app state storage location
#[derive(Debug)]
pub struct AppConfig {
    pub app_state_file_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            app_state_file_path: PathBuf::from("./app_state_model.bin"),
        }
    }
}
