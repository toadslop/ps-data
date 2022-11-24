use common::{defaults::DEFAULT_LOG_CONIFG_PATH, keys::LOG_CONFIG_PATH_KEY};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Env {
    log_config_path: PathBuf,
}

impl Env {
    pub fn init() -> Self {
        Self {
            log_config_path: Self::init_log_config_path(),
        }
    }

    fn init_log_config_path() -> PathBuf {
        if let Ok(log_path) = std::env::var(LOG_CONFIG_PATH_KEY) {
            Path::new(&log_path).to_owned()
        } else {
            Path::new(DEFAULT_LOG_CONIFG_PATH).to_owned()
        }
    }

    pub fn get_config_path(self) -> PathBuf {
        self.log_config_path
    }
}
