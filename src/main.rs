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
    General(String),
    ArgumentError(String),
    ReqwestError(String, reqwest::Error),
    ResponseError(String, reqwest::Response),
    IoError(String, std::io::Error),
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
    /// Path to .sideko file containing api key, default locations: ./.sideko then $HOME/.sideko
    config: Option<Utf8PathBuf>,
    #[arg(
        long,
        short = 'q',
        global = true,
        help = "No logging except for errors"
    )]
    quiet: bool,
    #[arg(long, short = 'v', global = true, help = "Verbose logging")]
    verbose: bool,
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
        #[arg(long, short)]
        /// Output path of generated source files, default: ./
        output: Option<Utf8PathBuf>,
        #[arg(long, short)]
        /// Base URL of API if not specified in OpenAPI spec
        base_url: Option<String>,
        #[arg(long, short)]
        /// Name of SDK package to generate
        package_name: Option<String>,
    },
}

async fn cli() -> CliResult<()> {
    let cli = Cli::parse();

    // set up logger
    let level = if cli.quiet {
        log::Level::Error
    } else if cli.verbose {
        log::Level::Debug
    } else {
        log::Level::Info
    };
    utils::init_logger(level);

    match &cli.command {
        Commands::Generate {
            openapi_path,
            language,
            output,
            base_url,
            package_name,
        } => {
            log::info!(
                "Generating Sideko SDK in {}",
                &language.to_string().to_uppercase()
            );
            utils::load_config(utils::config_bufs(vec![cli.config]))?;

            // Input validation
            if let Some(base_url) = &base_url {
                utils::validate_url(base_url)?;
            }

            let output = if let Some(o) = output {
                o.clone()
            } else {
                let cwd = std::env::current_dir().map_err(|e| {
                    log::debug!("CWD failure: {e}");
                    CliError::General("Failed determining cwd for --output default".to_string())
                })?;
                Utf8PathBuf::from_path_buf(cwd).map_err(|_| {
                    CliError::General("Unable to build default --output path".to_string())
                })?
            };

            utils::validate_path(openapi_path, &utils::PathKind::File, false)?;
            utils::validate_path(&output, &utils::PathKind::Dir, true)?;

            let ext = &openapi_path.extension().ok_or(CliError::ArgumentError(
                "Invalid file extension".to_string(),
            ))?;

            // generate sdk
            let bytes = cmds::generate::handle_generate(
                openapi_path,
                ext,
                language,
                base_url,
                package_name,
            )
            .await?;

            // save to output path
            let gz_decoder = GzDecoder::new(Cursor::new(&bytes));
            let mut archive = Archive::new(gz_decoder);
            if let Err(e) = archive.unpack(&output) {
                return Err(CliError::ArgumentError(format!(
                    "Failed to unpack archive: {}",
                    e
                )));
            }
            log::info!("Successfully generated SDK, saved to {output}");
        }
        Commands::Login { output } => {
            // Handle options
            let output_path = if let Some(o) = output {
                o.clone()
            } else {
                let home = std::env::var("HOME")
                    .map_err(|_| CliError::General("Unable to build default output path: $HOME is not set. Set environment variable or specify --output".to_string()))?;
                let mut utf_buf = Utf8PathBuf::from_str(&home).map_err(|_| {
                    CliError::General("Unable to build default --output path".to_string())
                })?;
                utf_buf.push(".sideko");
                utf_buf
            };

            // Validate input
            utils::validate_path(&output_path, &utils::PathKind::File, true)?;

            cmds::login::handle_login(&output_path).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    match cli().await {
        Err(CliError::ArgumentError(message) | CliError::General(message)) => {
            log::error!("{message}")
        }
        Err(CliError::ReqwestError(message, err)) => {
            log::debug!("{err}");
            log::error!("{message}");
        }
        Err(CliError::ResponseError(message, res)) => {
            log::debug!("Error response: {:?}", res);
            log::error!("{message}");
        }
        Err(CliError::IoError(message, err)) => {
            log::debug!("{err}");
            log::error!("{message}");
        }
        Ok(_) => (),
    }
}
