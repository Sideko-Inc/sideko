use std::{path::PathBuf, str::FromStr};

use url::Url;

use crate::result::{Error, Result};

pub static API_KEY_ENV_VAR: &str = "SIDEKO_API_KEY";

pub fn init_logger(level: log::Level) {
    if level == log::Level::Trace {
        env_logger::Builder::new().init();
    } else if level > log::Level::Info {
        env_logger::Builder::new()
            .filter_module("sideko", level.to_level_filter())
            .init();
    } else {
        env_logger::Builder::new()
            .filter_module("sideko", level.to_level_filter())
            .format_target(false)
            .format_timestamp(None)
            .init();
    }
}

pub fn sideko_base_url() -> String {
    let url = std::env::var("SIDKEO_BASE_URL").unwrap_or("https://api.sideko.dev".to_string());
    if url.ends_with('/') {
        url[0..url.len() - 1].to_string()
    } else {
        url
    }
}

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
pub fn load_config(bufs: Vec<PathBuf>) -> Result<()> {
    for buf in &bufs {
        let path_str = buf.to_str().unwrap_or_default();
        if !buf.is_file() {
            log::debug!("no config found at {path_str}");
            continue;
        }
        match dotenv::from_path(buf) {
            Ok(_) => {
                log::debug!("loaded config from {path_str}");
                return Ok(());
            }
            Err(_) => log::debug!("failed loading config from {path_str}"),
        };
    }

    Err(Error::ArgumentError(format!(
        "Failed loading config, no config file present in paths: {}",
        bufs.iter()
            .map(|b| b.to_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )))
}

/// Loads API key from environment
pub fn get_api_key() -> Result<String> {
    std::env::var(API_KEY_ENV_VAR).map_err(|_| {
        Error::ArgumentError(format!(
            "Failed loading Sideko API key, ensure {API_KEY_ENV_VAR} is set in your config file, or run `sideko login` to create it"
        ))
    })
}

/// Validates string is a valid URL
pub fn validate_url(val: &str) -> Result<Url> {
    url::Url::parse(val)
        .map_err(|_| Error::ArgumentError(format!("URL `{val}` is not a valid URL")))
}

pub enum PathKind {
    File,
    Dir,
}

/// Validates path kind & if it exists (optionally)
pub fn validate_path(buf: PathBuf, path_kind: &PathKind, allow_dne: bool) -> Result<()> {
    let path_str = buf.to_str().unwrap_or_default();
    let (allowed, err_msg) = match (path_kind, allow_dne) {
        (PathKind::File, false) => (
            buf.is_file(),
            format!("Path `{path_str}` must be a file or a non-existent path"),
        ),
        (PathKind::File, true) => (
            buf.is_file() || !buf.exists(),
            format!("Path `{path_str}` must be a file or a non-existent path"),
        ),
        (PathKind::Dir, false) => (
            buf.is_dir(),
            format!("Path `{path_str}` must be a directory or a non-existent path"),
        ),
        (PathKind::Dir, true) => (
            buf.is_dir() || !buf.exists(),
            format!("Path `{path_str}` must be a directory or a non-existent path"),
        ),
    };

    if allowed {
        Ok(())
    } else {
        Err(Error::ArgumentError(err_msg))
    }
}
