use crate::result::CliResult;

mod create;
mod list;
pub(crate) mod tabled;
mod update;

#[derive(clap::Subcommand)]
pub enum ApiVersionSubcommand {
    /// create a new version of an api with an openapi spec
    Create(create::ApiVersionCreateCommand),
    /// list api versions
    List(list::ApiVersionListCommand),
    /// updates an existing api version
    Update(update::ApiVersionUpdateCommand),
}
impl ApiVersionSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            ApiVersionSubcommand::List(cmd) => cmd.handle().await,
            ApiVersionSubcommand::Create(cmd) => cmd.handle().await,
            ApiVersionSubcommand::Update(cmd) => cmd.handle().await,
        }
    }
}
