use common::{defaults::DEFAULT_LOG_CONIFG_PATH, keys::LOG_CONFIG_PATH_KEY};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Env {
    log_config_path: PathBuf,
}

impl Env {
    pub fn init() -> Self {
        let log_config_path = Self::init_log_config_path();
        log::trace!(
            "Env struct initializing with path {}",
            log_config_path.display()
        );

        Self { log_config_path }
    }

    fn init_log_config_path() -> PathBuf {
        log::trace!("Acquiring log path...");

        if let Ok(log_path) = std::env::var(LOG_CONFIG_PATH_KEY) {
            log::debug!(
                "Environment variable {} found. Using {}",
                LOG_CONFIG_PATH_KEY,
                &log_path
            );

            Path::new(&log_path).to_owned()
        } else {
            log::debug!(
                "Environment variable {} not found. Using defaut {}",
                LOG_CONFIG_PATH_KEY,
                DEFAULT_LOG_CONIFG_PATH
            );

            Path::new(DEFAULT_LOG_CONIFG_PATH).to_owned()
        }
    }

    pub fn get_config_path(self) -> PathBuf {
        log::trace!("Config path requested");

        self.log_config_path
    }
}
