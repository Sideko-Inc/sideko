use crate::result::CliResult;

mod my_api_key;

#[derive(clap::Subcommand)]
pub enum AccountSubcommand {
    // ------------ SUB-COMMANDS ------------
    /// Gets the value of your personal Sideko API key and pastes it to your clipboard
    GetMyApiKey(my_api_key::GetMyApiKeyCommand),
}

impl AccountSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            AccountSubcommand::GetMyApiKey(cmd) => cmd.handle().await,
        }
    }
}
