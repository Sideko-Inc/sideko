use crate::result::{Error, Result};
use std::{path::PathBuf, str::FromStr};

/// Loads default location of .sideko config files in order
pub fn config_bufs(user_defined: Vec<Option<PathBuf>>) -> Vec<PathBuf> {
    let cwd_config = {
        if let Ok(mut cwd) = std::env::current_dir() {
            cwd.push(".sideko");
            Some(cwd)
        } else {
            None
        }
    };

    let home_config = {
        if let Ok(home) = std::env::var("HOME") {
            if let Ok(mut buf) = PathBuf::from_str(&home) {
                buf.push(".sideko");
                Some(buf)
            } else {
                None
            }
        } else {
            None
        }
    };

    let mut bufs = user_defined.clone();
    bufs.extend([cwd_config, home_config]);

    bufs.iter().filter_map(|b| b.clone()).collect()
}

/// Loads env from first path buf that exists
pub fn load_config(bufs: Vec<PathBuf>) {
    for buf in &bufs {
        let path_str = buf.to_str().unwrap_or_default();
        if !buf.is_file() {
            log::debug!("no config found at {path_str}");
            continue;
        }

        match dotenv::from_path(buf) {
            Ok(_) => {
                log::debug!("loaded config from {path_str}");
                return;
            }
            Err(_) => log::debug!("failed loading config from {path_str}"),
        };
    }
}

pub static API_KEY_ENV_VAR: &str = "SIDEKO_API_KEY";
pub static API_BASE_URL_ENV_VAR: &str = "SIDKEO_BASE_URL";

pub fn get_base_url() -> String {
    let url = std::env::var(API_BASE_URL_ENV_VAR).unwrap_or("https://api.sideko.dev".to_string());
    if url.ends_with('/') {
        url[0..url.len() - 1].to_string()
    } else {
        url
    }
}

/// Loads API key from environment
pub fn get_api_key() -> Result<String> {
    std::env::var(API_KEY_ENV_VAR).map_err(|_| {
        Error::ArgumentError(format!(
            "Failed loading Sideko API key, ensure {API_KEY_ENV_VAR} is set in your environment or config file"
        ))
    })
}
