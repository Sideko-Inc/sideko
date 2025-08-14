use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::resources::sdk::metadata::UpdateRequest;

use crate::{
    cmds::sdk::SdkMetadata, result::CliResult, styles::fmt_green, utils::get_sideko_client,
};

#[derive(clap::Args)]
pub struct SdkReleasedCommand {
    /// path to root of sdk repo
    #[arg(long, default_value = "./", value_parser = crate::utils::validators::validate_dir)]
    pub repo: Utf8PathBuf,

    /// sdk id to be marked as released, if this is provided the command will not look for the sdk id via the --repo
    #[arg(long)]
    pub id: Option<String>,
}

impl SdkReleasedCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let sdk_id = if let Some(i) = &self.id {
            i.clone()
        } else {
            SdkMetadata::load_from_repo(&self.repo)?.id
        };

        let mut client = get_sideko_client();
        let updated = client
            .sdk()
            .metadata()
            .update(UpdateRequest {
                sdk_id,
                released: Some(true),
                ..Default::default()
            })
            .await?;

        info!(
            "{} {} v{} ({}) marked as released",
            fmt_green("✔"),
            &updated.name,
            &updated.version,
            &updated.language
        );

        Ok(())
    }
}
