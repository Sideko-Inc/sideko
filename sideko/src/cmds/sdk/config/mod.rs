use crate::result::CliResult;

pub(crate) mod init;
pub(crate) mod sync;

#[derive(clap::Subcommand)]
pub enum SdkConfigSubcommand {
    // ------------ COMMANDS ------------
    /// generate the default sdk configuration for an api
    Init(init::SdkConfigInitCommand),

    /// sync sdk configuration file with an api version
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
