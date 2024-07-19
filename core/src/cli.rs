use crate::{
    cmds::{
        self,
        generate::{load_openapi, OpenApiSource},
    },
    config, result, styles, utils,
};
use clap::{Parser, Subcommand, ValueEnum};
use sideko_api::schemas::{self as sideko_schemas, NewApiVersion};

use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub struct GenerationLanguageClap {
    inner: sideko_schemas::GenerationLanguageEnum,
}
impl ValueEnum for GenerationLanguageClap {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            GenerationLanguageClap {
                inner: sideko_schemas::GenerationLanguageEnum::Go,
            },
            GenerationLanguageClap {
                inner: sideko_schemas::GenerationLanguageEnum::Ruby,
            },
            GenerationLanguageClap {
                inner: sideko_schemas::GenerationLanguageEnum::Rust,
            },
            GenerationLanguageClap {
                inner: sideko_schemas::GenerationLanguageEnum::Typescript,
            },
            GenerationLanguageClap {
                inner: sideko_schemas::GenerationLanguageEnum::Python,
            },
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match &self.inner {
            sideko_schemas::GenerationLanguageEnum::Go => {
                Some(clap::builder::PossibleValue::new("go"))
            }
            sideko_schemas::GenerationLanguageEnum::Ruby => {
                Some(clap::builder::PossibleValue::new("ruby"))
            }
            sideko_schemas::GenerationLanguageEnum::Rust => {
                Some(clap::builder::PossibleValue::new("rust"))
            }
            sideko_schemas::GenerationLanguageEnum::Typescript => {
                Some(clap::builder::PossibleValue::new("typescript"))
            }
            sideko_schemas::GenerationLanguageEnum::Python => {
                Some(clap::builder::PossibleValue::new("python"))
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "Sideko CLI")]
#[command(author = "Team Sideko <team@sideko.dev>")]
#[command(about = "Authenticate & Generate SDKs with Sideko in seconds", long_about = None)]
#[command(version)]
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
        /// Path to file to store API key, default: $HOME/.sideko
        output: Option<PathBuf>,
    },
    /// Generate and configure SDK clients
    #[command(subcommand)]
    Sdk(SdkCommands),
    /// Manage API specifications
    #[command(subcommand)]
    Api(ApiCommands),
    /// Manage documentation projects
    #[command(subcommand)]
    Doc(DocCommands),
}

#[derive(Debug, Subcommand)]
enum SdkCommands {
    Generate {
        /// Path or URL of OpenAPI spec
        openapi_source: String,
        /// Programming language to generate
        language: GenerationLanguageClap,
        #[arg(long, short)]
        /// Output path of generated source files, default: ./
        output: Option<PathBuf>,
        #[arg(long, short)]
        /// Base URL of API if not specified in OpenAPI spec
        base_url: Option<String>,
        #[arg(long, short)]
        /// Name of SDK package to generate
        package_name: Option<String>,
        #[arg(long, short)]
        /// URL of Sideko Mock Server for generated testing suite
        tests_mock_server_url: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum ApiCommands {
    /// List the existing project titles and ids
    List {},
    /// Create a new project
    Create {
        openapi_source: String,
        semver: String,
        title: Option<String>,
        notes: Option<String>,
    },
    /// Upload a new version to an existing project
    NewVersion {
        api_id: uuid::Uuid,
        openapi_source: String,
        semver: String,
        /// Plain text or HTML notes about the new API specification
        notes: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum DocCommands {
    /// List all titles and ids
    List {},
    /// Trigger a deployment to preview or production
    Deploy {
        doc_project_id: uuid::Uuid,
        #[arg(long, short)]
        /// Flag to deploy to production environment. If not set, it will deploy to preview
        prod: bool,
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
        Commands::Sdk(sdk_command) => {
            match sdk_command {
                SdkCommands::Generate {
                    openapi_source,
                    language,
                    output,
                    base_url,
                    package_name,
                    tests_mock_server_url,
                } => {
                    // Set defaults
                    let destination = if let Some(o) = output {
                        o.clone()
                    } else {
                        std::env::current_dir().map_err(|e| {
                            log::debug!("CWD failure: {e}");
                            result::Error::general("Failed determining cwd for --output default")
                        })?
                    };

                    // Construct cmd input params
                    let params = cmds::generate::GenerateSdkParams {
                        source: cmds::generate::OpenApiSource::from(openapi_source),
                        destination,
                        language: language.inner.clone(),
                        base_url: base_url.clone(),
                        package_name: package_name.clone(),
                        tests_mock_server_url: tests_mock_server_url.clone(),
                    };

                    if let OpenApiSource::Raw(_) = params.source {
                        log::error!("Unable to parse OpenAPI as a URL or Path");
                        return Err(result::Error::general(
                            "Unable to parse OpenAPI as a URL or Path",
                        ));
                    };

                    cmds::generate::handle_generate(&params).await
                }
            }
        }
        Commands::Login { output } => {
            // Set defaults
            let output_path = if let Some(o) = output {
                o.clone()
            } else {
                let home = std::env::var("HOME")
                        .map_err(|_| result::Error::general("Unable to build default output path: $HOME is not set. Set environment variable or specify --output"))?;
                let mut utf_buf = PathBuf::from_str(&home)
                    .map_err(|_| result::Error::general("Unable to build default --output path"))?;
                utf_buf.push(".sideko");
                utf_buf
            };

            cmds::login::handle_login(output_path).await
        }
        Commands::Api(api_command) => match api_command {
            ApiCommands::List {} => cmds::apis::handle_list_apis().await,
            ApiCommands::Create {
                openapi_source,
                semver,
                title,
                notes,
            } => {
                cmds::apis::create_new_api_project(
                    &NewApiVersion {
                        semver: semver.clone(),
                        openapi: load_openapi(&cmds::generate::OpenApiSource::from(openapi_source))
                            .await?,
                        mock_server_enabled: Some(true), // default to turning on the mock server
                        notes: notes.clone(),
                    },
                    title.clone(),
                )
                .await
            }
            ApiCommands::NewVersion {
                api_id,
                openapi_source,
                semver,
                notes,
            } => {
                cmds::apis::create_new_api_project_version(
                    *api_id,
                    &NewApiVersion {
                        openapi: load_openapi(&cmds::generate::OpenApiSource::from(openapi_source))
                            .await?,
                        semver: semver.clone(),
                        mock_server_enabled: Some(true),
                        notes: notes.clone(),
                    },
                )
                .await
            }
        },
        Commands::Doc(doc_command) => match doc_command {
            DocCommands::List {} => cmds::docs::handle_list_docs().await,
            DocCommands::Deploy {
                doc_project_id,
                prod,
            } => cmds::docs::handle_deploy_docs(doc_project_id, prod).await,
        },
    };

    if let Err(e) = &cmd_res {
        if let Some(debug_msg) = e.debug_msg() {
            log::debug!("{debug_msg}");
        }
        log::error!("{}", e.error_msg());
    }

    cmd_res
}
