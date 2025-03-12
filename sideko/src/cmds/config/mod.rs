use crate::result::CliResult;

mod autocomplete;

#[derive(clap::Subcommand)]
pub enum ConfigSubcommand {
    // ------------ COMMANDS ------------
    /// writes shell completion for the cli to stdout
    Autocomplete(autocomplete::AutocompleteCommand),
}

impl ConfigSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ConfigSubcommand::Autocomplete(cmd) => cmd.handle().await,
        }
    }
}
