use arboard::Clipboard;
use log::info;

use crate::{
    result::{CliError, CliResult},
    styles::{fmt_green, fmt_yellow},
    utils::get_sideko_client,
};

#[derive(clap::Args)]
pub struct GetMyApiKeyCommand {}

impl GetMyApiKeyCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let api_key_res = client.user().me().get_key().await?;

        let mut clipboard = Clipboard::new()
            .map_err(|err| CliError::arboard_custom("could not create clipboard", err))?;
        clipboard.set_text(api_key_res.api_key).map_err(|err| {
            CliError::arboard_custom("could not copy test to your clipboard", err)
        })?;

        info!("{} api key set to clipboard.", fmt_green("✔"));
        info!(
            "{} save the key in a secure location.",
            fmt_yellow("⚠️ ⚠️ ⚠️")
        );
        Ok(())
    }
}
