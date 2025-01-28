use log::{debug, error, info, warn};
use sideko_rest_api::{
    models::CliUpdateSeverityEnum, resources::cli::CheckUpdatesRequest, SidekoClient,
};

use crate::result::{CliError, CliResult};

pub(crate) mod config;
pub(crate) mod editor;
pub(crate) mod logging;
pub(crate) mod response;
pub(crate) mod spinner;
pub(crate) mod tabled;
pub(crate) mod url_builder;
pub(crate) mod validators;

/// Initializes SidekoClient using base url & api key from config environment
pub(crate) fn get_sideko_client() -> SidekoClient {
    let mut client = SidekoClient::default().with_base_url(&config::get_base_url());
    if let Some(key) = config::get_api_key() {
        client = client.with_api_key_auth(&key);
    }

    client
}

/// Uses the Sideko API to check for CLI notices/update requirements
pub async fn check_for_updates() -> CliResult<()> {
    let cli_version = env!("CARGO_PKG_VERSION").to_string();
    debug!("Checking for updates (CLI version: {cli_version})...");

    let mut client = SidekoClient::default().with_base_url(&config::get_base_url());
    let updates = client
        .cli()
        .check_updates(CheckUpdatesRequest { cli_version })
        .await?;

    if updates.is_empty() {
        debug!("No updates!")
    } else {
        let mut early_exit = false;
        for update in updates {
            match update.severity {
                CliUpdateSeverityEnum::Info => {
                    info!("Update info: {}", update.message);
                }
                CliUpdateSeverityEnum::Suggested => {
                    warn!("Update suggested: {}", update.message);
                }
                CliUpdateSeverityEnum::Required => {
                    error!("Update required: {}", update.message);
                    early_exit = true;
                }
            }
        }

        if early_exit {
            return Err(CliError::general("Must update CLI to continue"));
        }
    }

    Ok(())
}

/// Capitalizes first char in provided string
pub fn capitalize(val: &str) -> String {
    let mut chars = val.chars();
    if let Some(first) = chars.next() {
        first.to_uppercase().to_string() + chars.as_str()
    } else {
        String::new()
    }
}
