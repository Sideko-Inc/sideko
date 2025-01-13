use crate::result::CliResult;

#[derive(clap::Args)]
pub struct DocDeployCommand {}
impl DocDeployCommand {
    pub async fn handle(&self) -> CliResult<()> {
        todo!()
    }
}
