use crate::result::CliResult;

mod create;
mod list;
mod stats;
mod tabled;
mod version;

#[derive(clap::Subcommand)]
pub enum ApiSubcommand {
    // ------------ SUB-COMMANDS ------------
    /// Manage API specification versions
    #[command(subcommand)]
    Version(version::ApiVersionSubcommand),

    // ------------ COMMANDS ------------
    /// Create a new API
    Create(create::ApiCreateCommand),
    /// List all APIs
    List(list::ApiListCommand),
    /// Display stats gathered from the API specification
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
