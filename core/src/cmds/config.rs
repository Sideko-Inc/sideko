use std::fs;

use crate::result::{Error, Result};
use camino::Utf8PathBuf;
use prettytable::{format, row, Table};
use sideko_rest_api::{
    models::{ApiVersion, InitSdkConfig, SyncSdkConfig},
    resources::sdk::config::{init::InitRequest, sync::SyncRequest},
    Client, UploadFile,
};

use crate::config;

pub async fn init(api_name: String, api_version: Option<ApiVersion>) -> Result<()> {
    let api_key = config::get_api_key()?;
    let client = Client::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);

    let config_response = client
        .sdk()
        .config()
        .init()
        .init(InitRequest {
            data: InitSdkConfig {
                api_name,
                api_version,
            },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed to initialize SDK Config. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;

    // Convert binary response to YAML
    let yaml_str = String::from_utf8(config_response.content.to_vec()).map_err(|e| {
        Error::general_with_debug(
            "Failed to parse yaml config response as UTF-8",
            &format!("{e}"),
        )
    })?;
    let truncated_yaml = match yaml_str.find("modules:") {
        Some(index) => &yaml_str[..index],
        None => &yaml_str,
    };

    // Save YAML to file
    fs::write("sdk-config.yaml", &yaml_str).map_err(|e| {
        Error::general_with_debug("Failed to write SDK config file", &format!("{e}"))
    })?;

    // Create and print table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Sideko SDK Configuration"]);
    table.add_row(row![truncated_yaml]);
    table.printstd();

    Ok(())
}

pub async fn sync(config: &Utf8PathBuf, api_version: Option<ApiVersion>) -> Result<()> {
    let api_key = config::get_api_key()?;
    let client = Client::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);

    let config_response = client
        .sdk()
        .config()
        .sync()
        .sync(SyncRequest {
            data: SyncSdkConfig {
                api_version,
                config: UploadFile::from_path(config.as_str()).unwrap(),
            },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed to sync SDK Config. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;

    // Convert binary response to YAML
    let yaml_str = String::from_utf8(config_response.content.to_vec()).map_err(|e| {
        Error::general_with_debug(
            "Failed to parse yaml config response as UTF-8",
            &format!("{e}"),
        )
    })?;

    // Truncate YAML for display
    let truncated_yaml = match yaml_str.find("modules:") {
        Some(index) => &yaml_str[..index],
        None => &yaml_str,
    };

    // Overwrite the original config file
    fs::write(config, &yaml_str).map_err(|e| {
        Error::general_with_debug(
            &format!("Failed to write synced config to {}", config),
            &format!("{e}"),
        )
    })?;

    // Create and print table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["New Sideko SDK Configuration"]);
    table.add_row(row![truncated_yaml]);
    table.printstd();

    Ok(())
}
