use crate::result::CliResult;

mod list;
mod stats;
mod version;

#[derive(clap::Subcommand)]
pub enum ApiSubcommand {
    // ------------ SUB-COMMANDS ------------
    /// Command group to manage your API versions
    #[command(subcommand)]
    Version(version::ApiVersionSubcommand),

    // ------------ COMMANDS ------------
    // TODO: create
    List(list::ApiListCommand),
    Stats(stats::ApiStatsCommand),
    // TODO: version updates etc?
}

impl ApiSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ApiSubcommand::List(cmd) => cmd.handle().await,
            ApiSubcommand::Stats(cmd) => cmd.handle().await,
            ApiSubcommand::Version(cmd) => cmd.handle().await,
        }
    }
}
