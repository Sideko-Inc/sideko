use crate::result::CliResult;

mod create;
mod lint;
mod list;
mod stats;
mod tabled;
mod version;

#[derive(clap::Subcommand)]
pub enum ApiSubcommand {
    // ------------ SUB-COMMANDS ------------
    /// manage api specification versions
    #[command(subcommand)]
    Version(version::ApiVersionSubcommand),

    // ------------ COMMANDS ------------
    /// create a new api
    Create(create::ApiCreateCommand),
    /// list all apis
    List(list::ApiListCommand),
    /// display stats gathered from the specification
    Stats(stats::ApiStatsCommand),
    /// linting errors gathered from the specification
    Lint(lint::LintCommand),
}

impl ApiSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ApiSubcommand::Version(cmd) => cmd.handle().await,
            ApiSubcommand::Create(cmd) => cmd.handle().await,
            ApiSubcommand::List(cmd) => cmd.handle().await,
            ApiSubcommand::Stats(cmd) => cmd.handle().await,
            ApiSubcommand::Lint(cmd) => cmd.handle().await,
        }
    }
}
