use crate::{cmds, config, result, styles, utils};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, str::FromStr};

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
    config: Option<PathBuf>,
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
        output: Option<PathBuf>,
    },
    /// Generate a SDK client
    Generate {
        /// Path or URL of OpenAPI spec
        openapi_source: String,
        /// Programming language to generate
        language: cmds::generate::ProgrammingLanguage,
        #[arg(long, short)]
        /// Output path of generated source files, default: ./
        output: Option<PathBuf>,
        #[arg(long, short)]
        /// Base URL of API if not specified in OpenAPI spec
        base_url: Option<String>,
        #[arg(long, short)]
        /// Name of SDK package to generate
        package_name: Option<String>,
    },
}

pub async fn cli(args: Vec<String>) -> result::Result<()> {
    let cli = Cli::parse_from(args);

    // set up logger
    let level = if cli.quiet {
        log::Level::Error
    } else if cli.verbose {
        log::Level::Debug
    } else {
        log::Level::Info
    };
    utils::init_logger(level);
    config::load_config(config::config_bufs(vec![cli.config]));

    let cmd_res = match &cli.command {
        Commands::Generate {
            openapi_source,
            language,
            output,
            base_url,
            package_name,
        } => {
            // Set defaults
            let destination = if let Some(o) = output {
                o.clone()
            } else {
                std::env::current_dir().map_err(|e| {
                    log::debug!("CWD failure: {e}");
                    result::Error::General(
                        "Failed determining cwd for --output default".to_string(),
                    )
                })?
            };

            // Construct cmd input params
            let params = cmds::generate::GenerateSdkParams {
                source: cmds::generate::OpenApiSource::from(openapi_source),
                destination,
                language: language.clone(),
                base_url: base_url.clone(),
                package_name: package_name.clone(),
            };

            cmds::generate::handle_generate(&params).await
        }
        Commands::Login { output } => {
            // Set defaults
            let output_path = if let Some(o) = output {
                o.clone()
            } else {
                let home = std::env::var("HOME")
                        .map_err(|_| result::Error::General("Unable to build default output path: $HOME is not set. Set environment variable or specify --output".to_string()))?;
                let mut utf_buf = PathBuf::from_str(&home).map_err(|_| {
                    result::Error::General("Unable to build default --output path".to_string())
                })?;
                utf_buf.push(".sideko");
                utf_buf
            };

            cmds::login::handle_login(output_path).await
        }
    };

    match &cmd_res {
        Err(result::Error::ArgumentError(message) | result::Error::General(message)) => {
            log::error!("{message}")
        }
        Err(result::Error::ReqwestError(message, err)) => {
            log::debug!("{err}");
            log::error!("{message}");
        }
        Err(result::Error::ResponseError(message, res)) => {
            log::debug!("Error response: {:?}", res);
            log::error!("{message}");
        }
        Err(result::Error::IoError(message, err)) => {
            log::debug!("{err}");
            log::error!("{message}");
        }
        Ok(_) => (),
    };

    cmd_res
}
