use std::str::FromStr;

use camino::Utf8PathBuf;
use url::Url;

use crate::CliResult;

pub(crate) static API_KEY_ENV_VAR: &str = "SIDEKO_API_KEY";

pub(crate) fn sideko_base_url() -> String {
    // let url = std::env::var("SIDKEO_BASE_URL").unwrap_or("https://api.sideko.dev".to_string());
    let url = std::env::var("SIDKEO_BASE_URL").unwrap_or("http://localhost:8080".to_string());
    if url.ends_with('/') {
        url[0..url.len() - 1].to_string()
    } else {
        url
    }
}

/// Loads default location of .sideko config files in order
pub(crate) fn config_bufs(user_defined: Vec<Option<Utf8PathBuf>>) -> Vec<Utf8PathBuf> {
    let cwd_config = {
        if let Ok(cwd) = std::env::current_dir() {
            if let Ok(mut buf) = Utf8PathBuf::from_path_buf(cwd) {
                buf.push(".sideko");
                Some(buf)
            } else {
                None
            }
        } else {
            None
        }
    };

    let home_config = {
        if let Ok(home) = std::env::var("HOME") {
            if let Ok(mut buf) = Utf8PathBuf::from_str(&home) {
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
pub(crate) fn load_config(bufs: Vec<Utf8PathBuf>) -> CliResult<()> {
    for buf in &bufs {
        if !buf.is_file() {
            println!("no config found at {buf}");
            continue;
        }
        match dotenv::from_path(buf) {
            Ok(_) => {
                println!("loaded config from {buf}");
                return Ok(());
            }
            Err(_) => println!("failed loading config from {buf}"),
        };
    }

    Err(crate::CliError::ArgumentError(format!(
        "Failed loading config, no config file present in paths: {}",
        bufs.iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )))
}

/// Loads API key from environment
pub(crate) fn get_api_key() -> CliResult<String> {
    std::env::var(API_KEY_ENV_VAR).map_err(|_| {
        crate::CliError::ArgumentError(format!(
            "Failed loading Sideko API key, ensure {API_KEY_ENV_VAR} is set in your config file, or run `sideko login` to create it"
        ))
    })
}

/// Validates string is a valid URL
pub(crate) fn validate_url(val: &str) -> CliResult<Url> {
    url::Url::parse(val)
        .map_err(|_| crate::CliError::ArgumentError(format!("URL `{val}` is not a valid URL")))
}

pub(crate) enum PathKind {
    File,
    Dir,
}

/// Validates path kind & if it exists (optionally)
pub(crate) fn validate_path(
    buf: &Utf8PathBuf,
    path_kind: &PathKind,
    allow_dne: bool,
) -> CliResult<()> {
    let (allowed, err_msg) = match (path_kind, allow_dne) {
        (PathKind::File, false) => (
            buf.is_file(),
            format!("Path `{buf}` must be a file or a non-existent path"),
        ),
        (PathKind::File, true) => (
            buf.is_file() || !buf.exists(),
            format!("Path `{buf}` must be a file or a non-existent path"),
        ),
        (PathKind::Dir, false) => (
            buf.is_dir(),
            format!("Path `{buf}` must be a directory or a non-existent path"),
        ),
        (PathKind::Dir, true) => (
            buf.is_dir() || !buf.exists(),
            format!("Path `{buf}` must be a directory or a non-existent path"),
        ),
    };

    if allowed {
        Ok(())
    } else {
        Err(crate::CliError::ArgumentError(err_msg))
    }
}
