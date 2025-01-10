use crate::result::CliResult;

mod config;

#[derive(clap::Subcommand)]
pub enum SdkSubcommand {
    // ------------ SUB-COMMANDS ------------
    /// Command group for managing your SDK configs
    #[command(subcommand)]
    Config(config::SdkConfigSubcommand), // ------------ COMMANDS ------------
}

impl SdkSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            SdkSubcommand::Config(cmd) => cmd.handle().await,
        }
    }
}
