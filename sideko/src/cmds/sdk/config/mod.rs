use crate::result::CliResult;

pub(crate) mod init;
pub(crate) mod sync;

#[derive(clap::Subcommand)]
pub enum SdkConfigSubcommand {
    // ------------ COMMANDS ------------
    /// Generate the default SDK configuration for an API
    Init(init::SdkConfigInitCommand),

    /// Sync SDK configuration file with an API version
    Sync(sync::SdkConfigSyncCommand),
}

impl SdkConfigSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            SdkConfigSubcommand::Init(cmd) => cmd.handle().await,
            SdkConfigSubcommand::Sync(cmd) => cmd.handle().await,
        }
    }
}
