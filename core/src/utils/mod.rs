use log::{debug, error, info, warn};
use sideko_rest_api::{resources::cli::CheckUpdatesRequest, SidekoClient};

use crate::result::{CliError, CliResult};

pub(crate) mod config;
pub(crate) mod logging;
pub(crate) mod tabled;
pub(crate) mod validators;

pub(crate) fn get_sideko_client() -> SidekoClient {
    let mut client = SidekoClient::default().with_base_url(&config::get_base_url());
    if let Some(key) = config::ConfigKey::ApiKey.get() {
        client = client.with_api_key_auth(&key)
    }

    client
}

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
                sideko_rest_api::models::CliUpdateSeverityEnum::Info => {
                    info!("Update info: {}", update.message);
                }
                sideko_rest_api::models::CliUpdateSeverityEnum::Suggested => {
                    warn!("Update suggested: {}", update.message);
                }
                sideko_rest_api::models::CliUpdateSeverityEnum::Required => {
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
