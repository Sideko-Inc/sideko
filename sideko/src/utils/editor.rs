use crate::result::{CliError, CliResult};
use camino::Utf8PathBuf;
use inquire::Confirm;
use log::{debug, info};
use std::env;
use std::process::Command;

pub fn get_editor() -> String {
    // Match Git's precedence for editor selection
    env::var("GIT_EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| {
            if cfg!(target_os = "windows") {
                "notepad".to_string()
            } else {
                "vi".to_string()
            }
        })
}

pub fn open_config_in_editor(config_path: &Utf8PathBuf) -> CliResult<()> {
    let editor = get_editor();
    debug!("Using editor: {}", editor);
    info!("Opening editor for file: {} - please review the SDK config and save any changes before closing", config_path);

    Command::new(&editor)
        .arg(config_path.as_str())
        .status()
        .map_err(|e| {
            CliError::io_custom(
                format!("Failed to open '{}' in editor: {}", config_path, e),
                e,
            )
        })?;

    let confirmed = Confirm::new("Have you completed reviewing the SDK config?")
        .with_default(true)
        .with_help_message("'n' to open the SDK config again")
        .prompt()?;

    if !confirmed {
        return open_config_in_editor(config_path);
    }

    info!("SDK config review complete");
    Ok(())
}
