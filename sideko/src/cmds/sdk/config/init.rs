use std::fs;

use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::{models::ApiVersion, resources::sdk::config::InitRequest};

use crate::{
    cmds::sdk::SdkModuleStructure,
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

#[derive(clap::Args)]
pub struct SdkConfigInitCommand {
    /// api name or id e.g. my-api
    #[arg(long)]
    pub api_name: String,

    /// generate config for specific version (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    pub api_version: String,

    /// default module structure that should be generated
    /// for the SDK config.
    #[arg(long)]
    pub module_structure: Option<SdkModuleStructure>,

    /// custom output path of sdk config (must be .yaml or .yml)
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

        let config_res = client
            .sdk()
            .config()
            .init(InitRequest {
                api_name: self.api_name.clone(),
                api_version: Some(ApiVersion::Str(self.api_version.clone())),
                default_module_structure: self.module_structure.clone().map(|m| m.0),
            })
            .await?;

        // load yml as string and save to output
        let config = String::from_utf8(config_res.content.to_vec()).map_err(|e| {
            CliError::general_debug(
                "failed to parse config yaml as UTF-8 string",
                format!("{e:?}"),
            )
        })?;
        fs::write(&self.output, &config).map_err(|e| {
            CliError::io_custom(format!("failed writing config to {}", &self.output), e)
        })?;

        // preview the config
        info!("config written to {}", &self.output);
        utils::logging::log_table(utils::tabled::preview_table(
            "sdk configuration Preview",
            &config,
            15,
        ));

        Ok(())
    }
}
