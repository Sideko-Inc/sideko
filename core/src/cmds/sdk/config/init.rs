use std::fs;

use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::{
    models::{ApiVersion, ConfigCustomizationsEnum},
    resources::sdk::config::InitRequest,
};

use crate::{
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

#[derive(clap::Args)]
pub struct SdkConfigInitCommand {
    /// API name or id e.g. my-api
    #[arg(long)]
    pub api_name: String,

    /// Generate config for specific version (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    pub api_version: String,

    /// Use the `x-sideko-*` x-fields in OpenAPI to define the module structure/function names for the SDK
    ///
    /// Including this flag will cause the module config to be omitted from the generated
    /// config file.
    #[arg(long)]
    pub x_mods: bool,

    /// Custom output path of SDK config (must be .yaml or .yml)
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_yaml_allow_dne,
        default_value = "./sdk-config.yaml",
    )]
    pub output: Utf8PathBuf,
}

impl SdkConfigInitCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let customizations = if self.x_mods {
            ConfigCustomizationsEnum::XField
        } else {
            ConfigCustomizationsEnum::Config
        };

        let config_res = client
            .sdk()
            .config()
            .init(InitRequest {
                api_name: self.api_name.clone(),
                api_version: Some(ApiVersion::Str(self.api_version.clone())),
                customizations: Some(customizations),
            })
            .await?;

        // load yml as string and save to output
        let config = String::from_utf8(config_res.content.to_vec()).map_err(|e| {
            CliError::general_debug(
                "Failed to parse config yaml as UTF-8 string",
                format!("{e:?}"),
            )
        })?;
        fs::write(&self.output, &config).map_err(|e| {
            CliError::io_custom(format!("Failed writing config to {}", &self.output), e)
        })?;

        // preview the config
        info!("Config written to {}", &self.output);
        utils::logging::log_table(utils::tabled::preview_table(
            "SDK Configuration Preview",
            &config,
            15,
        ));

        Ok(())
    }
}
