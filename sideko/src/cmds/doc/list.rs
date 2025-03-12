use tabled::settings::{object::Rows, Color};

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};

use super::tabled::TabledDocProject;

#[derive(clap::Args)]
pub struct DocListCommand {
    /// display result format
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl DocListCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let doc_projects = client.doc().list().await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&doc_projects),
            DisplayOutput::Pretty => {
                let org = client.org().get().await?;
                let mut table =
                    tabled::Table::new(doc_projects.into_iter().map(|doc| TabledDocProject {
                        doc,
                        org_subdomain: org.subdomain.clone(),
                    }));
                utils::tabled::header_panel(&mut table, "documentation projects");
                table.modify(Rows::single(1), Color::BOLD);
                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
