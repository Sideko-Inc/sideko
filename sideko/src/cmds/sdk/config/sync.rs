use std::fs;

use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::{models::ApiVersion, resources::sdk::config::SyncRequest, UploadFile};

use crate::{
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

#[derive(clap::Args)]
pub struct SdkConfigSyncCommand {
    /// sync config with specific api version (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    pub api_version: String,

    /// sync config with local openapi specification
    #[arg(long, value_parser = crate::utils::validators::validate_file_json_yaml)]
    pub spec: Option<Utf8PathBuf>,

    /// config to sync
    #[arg(long, value_parser = crate::utils::validators::validate_file_yaml)]
    pub config: Utf8PathBuf,

    /// custom output path of sdk config (must be .yaml or .yml) [defaults to same path as --config]
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_yaml_allow_dne,
    )]
    pub output: Option<Utf8PathBuf>,
}

impl SdkConfigSyncCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let (api_version, openapi) = if let Some(spec) = &self.spec {
            (
                None,
                Some(UploadFile::from_path(spec.as_str()).map_err(|e| {
                    CliError::io_custom(format!("failed reading openapi from path: {spec}"), e)
                })?),
            )
        } else {
            (Some(ApiVersion::Str(self.api_version.clone())), None)
        };

        let synced_res = client
            .sdk()
            .config()
            .sync(SyncRequest {
                api_version,
                config: UploadFile::from_path(self.config.as_str()).map_err(|e| {
                    CliError::io_custom(
                        format!("failed reading config from path: {}", &self.config),
                        e,
                    )
                })?,
                openapi,
            })
            .await?;

        // load yml as string and save to output
        let output = self.output.as_ref().unwrap_or(&self.config);
        let config = String::from_utf8(synced_res.content.to_vec()).map_err(|e| {
            CliError::general_debug(
                "failed to parse synced config yaml as UTF-8 string",
                format!("{e:?}"),
            )
        })?;
        fs::write(output, &config).map_err(|e| {
            CliError::io_custom(format!("failed writing synced config to {output}"), e)
        })?;

        // preview the synced config
        utils::logging::log_table(utils::tabled::preview_table(
            "sdk configuration preview",
            &config,
            25,
        ));

        info!("synced config written to {output}");

        Ok(())
    }
}
