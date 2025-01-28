use std::io::Cursor;

use camino::Utf8PathBuf;
use flate2::read::GzDecoder;

use log::{debug, info};
use sideko_rest_api::{models::ApiVersion, resources::sdk::GenerateRequest, UploadFile};
use spinoff::spinners;
use tar::Archive;

use crate::{
    result::{CliError, CliResult},
    utils::{
        self,
        {get_sideko_client, spinner::Spinner},
    },
};

use super::SdkLang;

#[derive(clap::Args)]
pub struct SdkCreateCommand {
    /// Path to SDK config
    #[arg(long, value_parser = crate::utils::validators::validate_file_yaml)]
    pub config: Utf8PathBuf,

    /// Programming language to generate
    #[arg(long)]
    pub lang: SdkLang,

    /// Semantic version of generated SDK
    #[arg(long, default_value = "0.1.0")]
    pub version: semver::Version,

    /// Generate SDK for a specific version of the API (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    pub api_version: String,

    /// Include Github actions for testing and publishing the SDK in the generation
    #[arg(long)]
    pub gh_actions: bool,

    /// Path to save SDK
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_dir_allow_dne,
        default_value = "./",
    )]
    pub output: Utf8PathBuf,
}

impl SdkCreateCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let start = chrono::Utc::now();

        let mut sp = Spinner::new(
            spinners::Circle,
            format!("🪄  Generating {} SDK", self.lang.0),
        );
        let sdk_res = match client
            .sdk()
            .generate(GenerateRequest {
                api_version: Some(ApiVersion::Str(self.api_version.clone())),
                config: UploadFile::from_path(self.config.as_str()).map_err(|e| {
                    CliError::io_custom(
                        format!("Failed reading config from path: {}", &self.config),
                        e,
                    )
                })?,
                github_actions: Some(self.gh_actions),
                language: self.lang.0.clone(),
                sdk_version: Some(self.version.to_string()),
            })
            .await
        {
            Ok(r) => {
                sp.stop_success(format!(
                    "{} {} SDK generated!",
                    self.lang.emoji(),
                    utils::capitalize(&self.lang.0.to_string())
                ));
                r
            }
            Err(e) => {
                sp.stop_error("Failed generating SDK");
                return Err(e.into());
            }
        };

        debug!(
            "Generation took {}s",
            (chrono::Utc::now() - start).num_seconds()
        );

        debug!(
            "Unpacking sdk to {dest}: {size} bytes",
            dest = &self.output,
            size = sdk_res.content.len(),
        );
        let decoder = GzDecoder::new(Cursor::new(&sdk_res.content));
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

        info!("Saved to {dest}");

        Ok(())
    }
}
