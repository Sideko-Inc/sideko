use crate::{cli::SidekoCli, result::CliResult};
use clap::CommandFactory;

#[derive(clap::Args)]
pub(crate) struct AutocompleteCommand {
    #[arg(long)]
    shell: clap_complete::Shell,
}

impl AutocompleteCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut cmd = SidekoCli::command();
        let cmd_name = cmd.get_name().to_string();
        clap_complete::generate(self.shell, &mut cmd, cmd_name, &mut std::io::stdout());

        Ok(())
    }
}
