use crate::result::CliResult;

mod init;

#[derive(clap::Subcommand)]
pub enum SdkConfigSubcommand {
    // ------------ COMMANDS ------------
    Init(init::SdkConfigInitCommand),
}

impl SdkConfigSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            SdkConfigSubcommand::Init(cmd) => cmd.handle().await,
        }
    }
}
