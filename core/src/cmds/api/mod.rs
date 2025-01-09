use crate::result::CliResult;

mod create;
mod list;
mod stats;
mod tabled;
mod version;

#[derive(clap::Subcommand)]
pub enum ApiSubcommand {
    // ------------ SUB-COMMANDS ------------
    /// Command group to manage your API versions
    #[command(subcommand)]
    Version(version::ApiVersionSubcommand),

    // ------------ COMMANDS ------------
    /// Create new API
    Create(create::ApiCreateCommand),
    /// List all APIs available to your user in the organization
    List(list::ApiListCommand),
    /// Display statistics about latest version of an API
    Stats(stats::ApiStatsCommand),
}

impl ApiSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ApiSubcommand::Version(cmd) => cmd.handle().await,
            ApiSubcommand::Create(cmd) => cmd.handle().await,
            ApiSubcommand::List(cmd) => cmd.handle().await,
            ApiSubcommand::Stats(cmd) => cmd.handle().await,
        }
    }
}
