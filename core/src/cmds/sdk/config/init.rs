use std::fs;

use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::{
    models::{ApiVersion, ConfigCustomizationsEnum},
    resources::sdk::config::InitRequest,
};
use tabled::{
    settings::{object::Rows, Remove},
    Table,
};

use crate::{
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

#[derive(clap::Args)]
pub struct SdkConfigInitCommand {
    /// API name or id e.g. my-api
    #[arg(long)]
    name: String,

    /// Generate config for specific version (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    version: String,

    /// Use the `x-sideko-*` x-fields in OpenAPI to define the module structure/function names for the SDK
    ///
    /// Including this flag will cause the module config to be omitted from the generated
    /// config file.
    #[arg(long)]
    x_mods: bool,

    /// Custom output path of SDK config (must be .yaml or .yml)
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_yaml_dne,
        default_value = "./sdk-config.yaml",
    )]
    output: Utf8PathBuf,
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
                api_name: self.name.clone(),
                api_version: Some(ApiVersion::Str(self.version.clone())),
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
        let cfg_split: Vec<String> = config.split("\n").map(String::from).collect();
        let mut cfg_preview = cfg_split[0..cfg_split.len().min(25)].to_vec();
        if cfg_preview.len() < cfg_split.len() {
            cfg_preview.push("...".into())
        }

        let mut table = Table::new([cfg_preview.join("\n")]);
        table.with(Remove::row(Rows::first()));
        utils::tabled::header_panel(&mut table, "SDK Configuration Preview");
        utils::logging::log_table(table);

        info!("Config written to {}", &self.output);

        Ok(())
    }
}
