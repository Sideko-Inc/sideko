use std::path::PathBuf;

use log::{debug, info, warn};
use sideko_api::{
    request_types as sideko_request_types, schemas as sideko_schemas, Client as SidekoClient,
};
use url::Url;

use crate::{
    config,
    result::{Error, Result},
};

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
    url::Url::parse(val).map_err(|_| Error::arg(&format!("URL `{val}` is not a valid URL")))
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
        Err(Error::arg(&err_msg))
    }
}

pub async fn check_for_updates() -> Result<()> {
    let cli_version = env!("CARGO_PKG_VERSION").to_string();
    debug!("Checking for updates (CLI verion: {cli_version})...");

    let client = SidekoClient::default().with_base_url(&config::get_base_url());
    let request = sideko_request_types::CliCheckUpdatesRequest { cli_version };
    let updates = client
        .cli_check_updates(request)
        .await
        .map_err(|e| info!("Failed checking for CLI updates: {}", e));

    let mut can_continue = true;
    if let Ok(updates) = updates {
        for update in updates {
            match update.severity {
                sideko_schemas::CliUpdateSeverityEnum::Info => {
                    info!("Update info: {}", update.message);
                }
                sideko_schemas::CliUpdateSeverityEnum::Suggested => {
                    warn!("Update suggested: {}", update.message);
                }
                sideko_schemas::CliUpdateSeverityEnum::Required => {
                    warn!("Update required: {}", update.message);
                    can_continue = false;
                }
            }
        }
    }

    if can_continue {
        Ok(())
    } else {
        Err(Error::general("Must update CLI to continue"))
    }
}
