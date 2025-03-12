use crate::{result::CliResult, styles::fmt_green, utils::config::ConfigKey};
use log::info;

#[derive(clap::Args)]
pub(crate) struct LogoutCommand {}

impl LogoutCommand {
    pub async fn handle(&self) -> CliResult<()> {
        std::env::remove_var(ConfigKey::ApiKey.to_string());
        ConfigKey::ApiKey.unset_env()?;
        ConfigKey::ApiKey.unset_keyring()?;

        info!("{} logout successful", fmt_green("✔"));
        Ok(())
    }
}
