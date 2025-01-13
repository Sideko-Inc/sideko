use crate::result::CliResult;

mod deploy;
mod list;
mod tabled;

#[derive(clap::Subcommand)]
pub enum DocSubcommand {
    // ------------ COMMANDS ------------
    /// List all documentation projects available to your user in the organization
    List(list::DocListCommand),
    /// Trigger documentation deployment to preview or production
    Deploy(deploy::DocDeployCommand),
}

impl DocSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            DocSubcommand::List(cmd) => cmd.handle().await,
            DocSubcommand::Deploy(cmd) => cmd.handle().await,
        }
    }
}
