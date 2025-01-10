use std::io::Cursor;

use camino::Utf8PathBuf;
use flate2::read::GzDecoder as ReadGzDecoder;

use log::{debug, info};
use sideko_rest_api::{models::ApiVersion, resources::sdk::GenerateRequest, UploadFile};
use tar::Archive;

use crate::{
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

use super::SdkLang;

#[derive(clap::Args)]
pub struct SdkCreateCommand {
    /// Path to SDK config
    #[arg(long, value_parser = crate::utils::validators::validate_file_yaml)]
    config: Utf8PathBuf,

    /// Programming language to generate
    #[arg(long)]
    lang: SdkLang,

    /// Generate SDK with for a specific version of the API listed in the config (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    version: String,

    /// Semantic version of generated SDK
    #[arg(long, default_value = "0.1.0")]
    sdk_version: semver::Version,

    /// Include Github actions for testing and publishing the SDK in the generation
    #[arg(long)]
    gh_actions: bool,

    /// Path to save SDK
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_dir_allow_dne,
        default_value = "./",
    )]
    output: Utf8PathBuf,
}

impl SdkCreateCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        info!("🪄 Generating {} SDK...", self.lang.0.to_string());
        let sdk_res = client
            .sdk()
            .generate(GenerateRequest {
                api_version: Some(ApiVersion::Str(self.version.clone())),
                config: UploadFile::from_path(self.config.as_str()).map_err(|e| {
                    CliError::io_custom(
                        format!("Failed reading config from path: {}", &self.config),
                        e,
                    )
                })?,
                github_actions: Some(self.gh_actions),
                language: self.lang.0.clone(),
                sdk_version: Some(self.sdk_version.to_string()),
            })
            .await?;

        // unpack into destination
        debug!(
            "Generation complete, unpacking sdk to {dest}: {size} bytes",
            dest = &self.output,
            size = sdk_res.content.len(),
        );
        let decoder = ReadGzDecoder::new(Cursor::new(&sdk_res.content));
        let mut archive = Archive::new(decoder);
        archive
            .unpack(&self.output)
            .map_err(|e| CliError::io_custom("Failed unpacking sdk archive into output", e))?;

        let mut dest = self.output.clone();
        if let Some(archive_filename) =
            utils::response::extract_filename(&sdk_res).map(String::from)
        {
            dest = dest.join(
                archive_filename
                    .strip_suffix(".tar.gz")
                    .unwrap_or(&archive_filename),
            )
        }
        info!("🚀 SDK saved to {dest}");

        Ok(())
    }
}
