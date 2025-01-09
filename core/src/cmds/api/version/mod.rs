use crate::result::CliResult;

mod list;
mod tabled;
#[derive(clap::Subcommand)]
pub enum ApiVersionSubcommand {
    List(list::ApiVersionListCommand),
}
impl ApiVersionSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ApiVersionSubcommand::List(cmd) => cmd.handle().await,
        }
    }
}
