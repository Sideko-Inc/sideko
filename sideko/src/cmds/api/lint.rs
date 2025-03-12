use std::fmt::Display;

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};
use sideko_rest_api::{
    models::{ApiVersion, LintErrorDetails},
    resources::api::spec::GetStatsRequest,
};
use tabled::settings::{object::Rows, Modify, Padding, Remove, Style, Width};

#[derive(clap::Args, Debug)]
pub struct LintCommand {
    /// aPI name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// api version e.g. v1, latest
    #[arg(long, default_value = "latest")]
    pub version: String,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}

impl LintCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let stats = client
            .api()
            .spec()
            .get_stats(GetStatsRequest {
                api_name: self.name.clone(),
                api_version: ApiVersion::Str(self.version.clone()),
            })
            .await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&stats.lint_errors),
            DisplayOutput::Pretty => {
                let lint_errors = stats.lint_errors;

                // Display summary
                let summary_data = vec![
                    SummaryRow::new(
                        "missing operation ids",
                        lint_errors.missing_operation_ids.len(),
                    ),
                    SummaryRow::new("incorrect paths", lint_errors.incorrect_paths.len()),
                    SummaryRow::new("incorrect examples", lint_errors.incorrect_examples.len()),
                ];

                let mut summary_table = tabled::Table::new(summary_data);
                summary_table
                    .with(Style::modern())
                    .with(Padding::new(1, 1, 0, 0))
                    .with(Remove::row(Rows::first()))
                    .with(Modify::new(Rows::new(0..)).with(Width::wrap(60)));

                utils::tabled::header_panel(&mut summary_table, "lint summary");
                utils::logging::log_table(summary_table);

                // Display detailed errors if any exist
                if !lint_errors.missing_operation_ids.is_empty() {
                    display_lint_error_details(
                        "missing operation ids",
                        &lint_errors.missing_operation_ids,
                    );
                }

                if !lint_errors.incorrect_paths.is_empty() {
                    println!("\nincorrect paths:");
                    for path in lint_errors.incorrect_paths {
                        println!("  - {}", path);
                    }
                }

                if !lint_errors.incorrect_examples.is_empty() {
                    display_lint_error_details(
                        "incorrect examples",
                        &lint_errors.incorrect_examples,
                    );
                }
            }
        }

        Ok(())
    }
}

#[derive(tabled::Tabled)]
struct SummaryRow {
    name: String,
    val: String,
}

impl SummaryRow {
    pub fn new<N: ToString, V: Display>(name: N, val: V) -> Self {
        Self {
            name: name.to_string(),
            val: val.to_string(),
        }
    }
}

#[derive(tabled::Tabled)]
struct LintErrorRow {
    #[tabled(rename = "path")]
    path: String,
    #[tabled(rename = "method")]
    method: String,
    #[tabled(rename = "location")]
    location: String,
    #[tabled(rename = "message")]
    message: String,
}

fn display_lint_error_details(title: &str, errors: &[LintErrorDetails]) {
    if errors.is_empty() {
        return;
    }

    let error_rows: Vec<LintErrorRow> = errors
        .iter()
        .map(|error| LintErrorRow {
            path: error.path.clone(),
            method: error.method.clone(),
            location: error.location.clone().unwrap_or_default(),
            message: error.message.clone().unwrap_or_default(),
        })
        .collect();

    let mut error_table = tabled::Table::new(error_rows);
    error_table
        .with(Style::modern())
        .with(Padding::new(1, 1, 0, 0))
        .with(Modify::new(Rows::new(0..)).with(Width::wrap(60)));

    utils::tabled::header_panel(&mut error_table, title);
    utils::logging::log_table(error_table);
}
