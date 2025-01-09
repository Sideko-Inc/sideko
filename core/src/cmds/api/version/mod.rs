use crate::result::CliResult;

mod create;
mod list;
pub(crate) mod tabled;
mod update;

#[derive(clap::Subcommand)]
pub enum ApiVersionSubcommand {
    /// Create a new version of an API with an OpenAPI spec
    Create(create::ApiVersionCreateCommand),
    /// List an APIs versions
    List(list::ApiVersionListCommand),
    /// Updates an existing API version,
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
