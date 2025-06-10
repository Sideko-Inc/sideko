use std::io::Cursor;

use camino::Utf8PathBuf;
use flate2::read::GzDecoder;

use log::{debug, info};
use sideko_rest_api::{models::ApiVersion, resources::sdk::GenerateRequest, UploadFile};
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
    /// path to sdk config
    #[arg(long, value_parser = crate::utils::validators::validate_file_yaml)]
    pub config: Utf8PathBuf,

    /// programming language to generate
    #[arg(long)]
    pub lang: SdkLang,

    /// semantic version of generated sdk
    #[arg(long, default_value = "0.1.0")]
    pub version: semver::Version,

    /// generate sdk for a specific version of the api (e.g. `2.1.5`)
    #[arg(long, default_value = "latest")]
    pub api_version: String,

    /// include github actions for testing and publishing the sdk in the generation
    #[arg(long)]
    pub gh_actions: bool,

    /// create the SDK even thought OpenAPI linting errors were caught in this API version
    #[arg(long)]
    pub allow_lint_errors: bool,

    /// path to save sdk
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

        let mut sp = Spinner::new(format!("🪄  generating {} sdk", self.lang.0));
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
                allow_lint_errors: Some(self.allow_lint_errors),
            })
            .await
        {
            Ok(r) => {
                sp.stop_success(format!(
                    "{} {} sdk generated.",
                    self.lang.emoji(),
                    &self.lang.0.to_string()
                ));
                r
            }
            Err(e) => {
                sp.stop_error("failed generating sdk");
                return Err(e.into());
            }
        };

        debug!(
            "generation took {}s",
            (chrono::Utc::now() - start).num_seconds()
        );

        debug!(
            "unpacking sdk to {dest}: {size} bytes",
            dest = &self.output,
            size = sdk_res.content.len(),
        );
        let decoder = GzDecoder::new(Cursor::new(&sdk_res.content));
        let mut archive = Archive::new(decoder);
        archive
            .unpack(&self.output)
            .map_err(|e| CliError::io_custom("failed unpacking sdk archive into output", e))?;

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

        info!("saved to {dest}");

        Ok(())
    }
}
