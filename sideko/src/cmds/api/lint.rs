use crate::{
    cmds::DisplayOutput,
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};
use camino::Utf8PathBuf;
use sideko_rest_api::{
    models::{ApiVersion, LintSeverityEnum},
    resources::lint::RunRequest,
    UploadFile,
};
use tabled::settings::{location::ByContent, object::Rows, Color, Modify};

use super::tabled::TabledLintResult;

#[derive(clap::Args, Debug)]
pub struct LintCommand {
    /// Path to local OpenAPI file to lint
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_json_yaml,
    )]
    pub spec: Option<Utf8PathBuf>,

    /// API name or id e.g. my-api
    #[arg(long)]
    pub name: Option<String>,

    /// API version e.g. v1, latest
    #[arg(long, default_value = "latest")]
    pub version: Option<String>,

    /// Show errors only
    #[arg(long)]
    pub errors: bool,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,

    /// save result as a CSV
    #[arg(long)]
    pub save: bool,
}

impl LintCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let mut report = match (&self.spec, &self.name, &self.version) {
            (Some(spec_path), ..) => {
                let openapi = UploadFile::from_path(spec_path.as_str()).map_err(|e| {
                    CliError::io_custom(format!("failed reading openapi from path: {spec_path}"), e)
                })?;

                client
                    .lint()
                    .run(RunRequest {
                        openapi: Some(openapi),
                        ..Default::default()
                    })
                    .await?
            }
            (_, Some(name), Some(version)) => {
                client
                    .lint()
                    .run(RunRequest {
                        api_name: Some(name.clone()),
                        api_version: Some(ApiVersion::Str(version.clone())),
                        ..Default::default()
                    })
                    .await?
            }
            _ => {
                return Err(CliError::general(
                    "you must either provide --spec <PATH> or --name <NAME> --version <VERSION>",
                ))
            }
        };
        if self.errors {
            report
                .results
                .retain(|result| matches!(&result.severity, LintSeverityEnum::Error));
        }

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&report),
            DisplayOutput::Pretty => {
                let filename =
                    if let Some(Some(filename)) = self.spec.as_ref().map(|p| p.file_name()) {
                        filename.to_string()
                    } else {
                        format!(
                            "{name}-{version}-openapi",
                            name = self.name.clone().unwrap_or_default(),
                            version = self.version.clone().unwrap_or_default()
                        )
                    };

                // build summary table
                let mut summary_rows: Vec<SummaryRow> = vec![];
                for result in &report.results {
                    if let Some(row) = summary_rows
                        .iter_mut()
                        .find(|r| r.category == result.category)
                    {
                        match &result.severity {
                            LintSeverityEnum::Error => row.errors += 1,
                            LintSeverityEnum::Warn => row.warnings += 1,
                            LintSeverityEnum::Info => row.info += 1,
                            LintSeverityEnum::Unknown => continue,
                        }
                    } else {
                        let new_row = match &result.severity {
                            LintSeverityEnum::Error => SummaryRow {
                                category: result.category.clone(),
                                errors: 1,
                                ..Default::default()
                            },
                            LintSeverityEnum::Warn => SummaryRow {
                                category: result.category.clone(),
                                warnings: 1,
                                ..Default::default()
                            },
                            LintSeverityEnum::Info => SummaryRow {
                                category: result.category.clone(),
                                info: 1,
                                ..Default::default()
                            },
                            LintSeverityEnum::Unknown => continue,
                        };
                        summary_rows.push(new_row);
                    }
                }
                summary_rows.push(SummaryRow {
                    category: "Total".into(),
                    errors: report.summary.errors as usize,
                    warnings: report.summary.warns as usize,
                    info: report.summary.infos as usize,
                });
                let mut summary_table = tabled::Table::new(summary_rows);
                utils::tabled::header_panel(
                    &mut summary_table,
                    &format!("{filename} Lint Summary"),
                );
                summary_table
                    .modify(Rows::single(1), Color::BOLD)
                    .modify(Rows::last(), Color::BOLD)
                    .with(Modify::new(ByContent::new("Errors")).with(Color::FG_RED))
                    .with(Modify::new(ByContent::new("Warnings")).with(Color::FG_YELLOW))
                    .with(Modify::new(ByContent::new("Info")).with(Color::FG_BLUE));

                if !&report.results.is_empty() {
                    let mut report_table =
                        tabled::Table::new(report.results.clone().into_iter().map(|result| {
                            Some(TabledLintResult {
                                filename: filename.to_string(),
                                result,
                            })
                        }));
                    utils::tabled::header_panel(
                        &mut report_table,
                        &format!("{filename} Lint Results"),
                    );
                    report_table
                        .with(Modify::new(ByContent::new("error")).with(Color::FG_RED))
                        .with(Modify::new(ByContent::new("warn")).with(Color::FG_YELLOW))
                        .with(Modify::new(ByContent::new("info")).with(Color::FG_BLUE));
                    report_table.modify(Rows::single(1), Color::BOLD);

                    utils::logging::log_table(report_table);
                }

                // display summary table
                utils::logging::log_table(summary_table);
            }
        }

        if self.save {
            let filename = if let Some(Some(filename)) = self.spec.as_ref().map(|p| p.file_name()) {
                filename.to_string()
            } else {
                format!(
                    "{name}-lint-report",
                    name = self.name.clone().unwrap_or_default(),
                )
            };
            let csv_filename = format!("{}.csv", filename);
            let file = std::fs::File::create(&csv_filename)
                .map_err(|e| CliError::io_custom(format!("Failed to create CSV file: {}", e), e))?;
            let mut wtr = csv::WriterBuilder::new().from_writer(file);
            #[derive(serde::Serialize)]
            struct FlatLintResult {
                category: String,
                severity: String,
                message: String,
                path: String,
                start_line: i64,
                start_column: i64,
                end_line: i64,
                end_column: i64,
            }
            for result in &report.results {
                let location = result.location.clone();
                let flat_result = FlatLintResult {
                    category: result.category.clone(),
                    severity: result.severity.to_string(),
                    message: result.message.clone(),
                    path: location.path.clone(),
                    start_line: location.start_line,
                    start_column: location.start_column,
                    end_line: location.end_line,
                    end_column: location.end_column,
                };
                wtr.serialize(flat_result).map_err(|e| {
                    CliError::io_custom(format!("Failed to write CSV data: {}", e), e.into())
                })?;
            }
            wtr.flush()
                .map_err(|e| CliError::io_custom(format!("Failed to flush CSV data: {}", e), e))?;

            // Inform the user where the report was saved
            log::info!("Lint report saved to: {}", csv_filename);
        }

        if report.summary.errors > 0 {
            Err(CliError::general(format!(
                "{} linting errors found",
                report.summary.errors
            )))
        } else {
            Ok(())
        }
    }
}

#[derive(tabled::Tabled, Default)]
struct SummaryRow {
    #[tabled(rename = "Category")]
    category: String,
    #[tabled(rename = "Errors")]
    errors: usize,
    #[tabled(rename = "Warnings")]
    warnings: usize,
    #[tabled(rename = "Info")]
    info: usize,
}
