use std::fs;

use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::{
    models::{ApiVersion, ConfigCustomizationsEnum},
    resources::sdk::config::SyncRequest,
    UploadFile,
};

use crate::{
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

#[derive(clap::Args)]
pub struct SdkConfigSyncCommand {
    /// API name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// Sync config with specific version (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    pub version: String,

    /// Sync config with local OpenAPI specification
    #[arg(long, value_parser = crate::utils::validators::validate_file_json_yaml)]
    pub spec: Option<Utf8PathBuf>,

    /// Config to sync
    #[arg(long, value_parser = crate::utils::validators::validate_file_yaml)]
    pub config: Utf8PathBuf,

    /// Custom output path of SDK config (must be .yaml or .yml) [defaults to same path as --config]
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_yaml_allow_dne,
    )]
    pub output: Option<Utf8PathBuf>,

    /// Use the `x-sideko-*` x-fields in OpenAPI to define the module structure/function names for the SDK
    ///
    /// Including this flag will cause the module config to be omitted from the generated
    /// config file.
    #[arg(long)]
    pub x_mods: bool,
}

impl SdkConfigSyncCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let customizations = if self.x_mods {
            ConfigCustomizationsEnum::XField
        } else {
            ConfigCustomizationsEnum::Config
        };

        let (api_version, openapi) = if let Some(spec) = &self.spec {
            (
                None,
                Some(UploadFile::from_path(spec.as_str()).map_err(|e| {
                    CliError::io_custom(format!("Failed reading OpenAPI from path: {spec}"), e)
                })?),
            )
        } else {
            (Some(ApiVersion::Str(self.version.clone())), None)
        };

        let synced_res = client
            .sdk()
            .config()
            .sync(SyncRequest {
                api_version,
                config: UploadFile::from_path(self.config.as_str()).map_err(|e| {
                    CliError::io_custom(
                        format!("Failed reading config from path: {}", &self.config),
                        e,
                    )
                })?,
                customizations: Some(customizations),
                openapi,
            })
            .await?;

        // load yml as string and save to output
        let output = self.output.as_ref().unwrap_or(&self.config);
        let config = String::from_utf8(synced_res.content.to_vec()).map_err(|e| {
            CliError::general_debug(
                "Failed to parses synced config yaml as UTF-8 string",
                format!("{e:?}"),
            )
        })?;
        fs::write(output, &config).map_err(|e| {
            CliError::io_custom(format!("Failed writing synced config to {output}"), e)
        })?;

        // preview the synced config
        utils::logging::log_table(utils::tabled::preview_table(
            "SDK Configuration Preview",
            &config,
            25,
        ));

        info!("Synced config written to {output}");

        Ok(())
    }
}
