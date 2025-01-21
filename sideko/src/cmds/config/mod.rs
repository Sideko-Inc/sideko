use crate::result::CliResult;

mod autocomplete;

#[derive(clap::Subcommand)]
pub enum ConfigSubcommand {
    // ------------ COMMANDS ------------
    /// Writes shell completion for the CLI to stdout
    ///
    /// Example (`zsh`): `sideko config autocomplete --shell zsh > ~/sideko-complete.sh`
    ///
    /// Then add `source ~/sideko-complete.sh` to `~/.zshrc`
    Autocomplete(autocomplete::AutocompleteCommand),
}

impl ConfigSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ConfigSubcommand::Autocomplete(cmd) => cmd.handle().await,
        }
    }
}
