use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

use flate2::read::GzDecoder;
use std::{io::Cursor, str::FromStr};
use tar::Archive;

mod cmds;
mod styles;
mod utils;

#[derive(Debug)]
pub enum CliError {
    ReqwestError(reqwest::Error),
    FailedResponse(reqwest::StatusCode, String),
    FileError(String),
    DownloadError(String),
    NetworkError(String),
    ArgumentError(String),
}

pub type CliResult<T> = std::result::Result<T, CliError>;

#[derive(Parser)]
#[command(name = "Sideko CLI")]
#[command(author = "Team Sideko <team@sideko.dev>")]
#[command(about = "Authenticate & Generate SDKs with Sideko in seconds", long_about = None)]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, short)]
    /// Path to .sideko file containing api key, default checks: $CWD/.sideko then $HOME/.sideko
    config: Option<Utf8PathBuf>,
}

#[derive(Subcommand)]
#[command(styles=styles::get_styles())]
enum Commands {
    /// Log into Sideko interactively to obtain API key for generations
    Login {
        #[arg(long, short)]
        /// Path to file to stored API key, default: $HOME/.sideko
        output: Option<Utf8PathBuf>,
    },
    /// Generate a SDK client
    Generate {
        /// Path to OpenAPI spec
        openapi_path: Utf8PathBuf,
        /// Programming language to generate
        language: cmds::generate::ProgrammingLanguage,
        /// Output path of generated source files
        output: Utf8PathBuf,
        #[arg(long, short)]
        /// Base URL of API if not specified in OpenAPI spec
        base_url: Option<String>,
        #[arg(long, short)]
        /// Name of SDK library to generate
        name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> CliResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            openapi_path,
            language,
            output,
            base_url,
            name,
        } => {
            println!(
                "Generating Sideko SDK in {}",
                &language.to_string().to_uppercase()
            );

            // Input validation
            if let Some(base_url) = &base_url {
                utils::validate_url(base_url)?;
            }

            utils::validate_path(openapi_path, &utils::PathKind::Dir, false)?;

            let ext = &openapi_path.extension().ok_or(CliError::ArgumentError(
                "Invalid file extension".to_string(),
            ))?;

            // generate sdk
            let bytes =
                cmds::generate::handle_generate(openapi_path, ext, language, base_url, name)?;

            // save to output path
            let gz_decoder = GzDecoder::new(Cursor::new(&bytes));
            let mut archive = Archive::new(gz_decoder);
            if let Err(e) = archive.unpack(output) {
                return Err(CliError::ArgumentError(format!(
                    "Failed to unpack archive: {}",
                    e
                )));
            }
            println!("Successfully generated SDK. Saving to {output}");
        }
        Commands::Login { output } => {
            // Handle options
            let output_path = if let Some(o) = output {
                o.clone()
            } else {
                let home = std::env::var("HOME")
                    .map_err(|_| CliError::ArgumentError("Unable to build default output path: $HOME is not set. Set environment variable or specify --output".to_string()))?;
                let mut utf_buff = Utf8PathBuf::from_str(&home).map_err(|_| {
                    CliError::FileError("Unable to build default output path".to_string())
                })?;
                utf_buff.push(".sideko");
                utf_buff
            };

            // Validate input
            utils::validate_path(&output_path, &utils::PathKind::File, true)?;

            cmds::login::handle_login(&output_path).await?;
        }
    }

    Ok(())
}
