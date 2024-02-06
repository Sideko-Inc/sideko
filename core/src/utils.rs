use std::path::PathBuf;

use url::Url;

use crate::result::{Error, Result};

pub fn init_logger(level: log::Level) {
    let _ = if level == log::Level::Trace {
        env_logger::Builder::new().try_init()
    } else if level > log::Level::Info {
        env_logger::Builder::new()
            .filter_module("sideko", level.to_level_filter())
            .try_init()
    } else {
        env_logger::Builder::new()
            .filter_module("sideko", level.to_level_filter())
            .format_target(false)
            .format_timestamp(None)
            .try_init()
    };
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
