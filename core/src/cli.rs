use crate::{
    cmds::{
        self,
        apis::data_list_versions,
        sdk::{load_openapi, OpenApiSource},
    },
    config,
    result::{self},
    styles, utils,
};
use clap::{Parser, Subcommand, ValueEnum};
use clap_markdown::MarkdownOptions;
use semver::Version;
use sideko_rest_api::models::{ApiSpec, NewApiSpec};

use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(name = "sideko")]
#[command(author = "Team Sideko <team@sideko.dev>")]
#[command(about = "Login to start generating tools for your APIs", long_about = None)]
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
    /// **Enterprise Only!**
    /// Manage API specifications
    #[command(subcommand)]
    Api(ApiCommands),
    /// **Enterprise Only!**
    /// Manage documentation projects
    #[command(subcommand)]
    Doc(DocCommands),
    /// Private command to generate CLI docs for the the Sideko CLI
    #[clap(hide = true)]
    MdDocs {
        #[arg(long)]
        save: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SemverIncrement {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Clone)]
pub enum SemverOrIncrement {
    Increment(SemverIncrement),
    Semver(String),
}

impl FromStr for SemverOrIncrement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "major" => Ok(SemverOrIncrement::Increment(SemverIncrement::Major)),
            "minor" => Ok(SemverOrIncrement::Increment(SemverIncrement::Minor)),
            "patch" => Ok(SemverOrIncrement::Increment(SemverIncrement::Patch)),
            _ => {
                // Assume it's a semver string if it's not one of the increment keywords
                Ok(SemverOrIncrement::Semver(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerationLanguageClap {
    inner: sideko_rest_api::models::GenerationLanguageEnum,
}
impl ValueEnum for GenerationLanguageClap {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            GenerationLanguageClap {
                inner: sideko_rest_api::models::GenerationLanguageEnum::Go,
            },
            GenerationLanguageClap {
                inner: sideko_rest_api::models::GenerationLanguageEnum::Ruby,
            },
            GenerationLanguageClap {
                inner: sideko_rest_api::models::GenerationLanguageEnum::Rust,
            },
            GenerationLanguageClap {
                inner: sideko_rest_api::models::GenerationLanguageEnum::Typescript,
            },
            GenerationLanguageClap {
                inner: sideko_rest_api::models::GenerationLanguageEnum::Python,
            },
            GenerationLanguageClap {
                inner: sideko_rest_api::models::GenerationLanguageEnum::Java,
            },
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match &self.inner {
            sideko_rest_api::models::GenerationLanguageEnum::Go => {
                Some(clap::builder::PossibleValue::new("go"))
            }
            sideko_rest_api::models::GenerationLanguageEnum::Ruby => {
                Some(clap::builder::PossibleValue::new("ruby"))
            }
            sideko_rest_api::models::GenerationLanguageEnum::Rust => {
                Some(clap::builder::PossibleValue::new("rust"))
            }
            sideko_rest_api::models::GenerationLanguageEnum::Typescript => {
                Some(clap::builder::PossibleValue::new("typescript"))
            }
            sideko_rest_api::models::GenerationLanguageEnum::Python => {
                Some(clap::builder::PossibleValue::new("python"))
            }
            sideko_rest_api::models::GenerationLanguageEnum::Java => {
                Some(clap::builder::PossibleValue::new("java"))
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum SdkCommands {
    /// Generate a point-in-time SDK (unmanaged/stateless). This command is available to free-tier users.
    Try {
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
    },
    /// **Enterprise Only!**
    /// Create a managed SDK that Sideko can track and maintain maintain. This command returns an SDK repo with git tracking
    Create {
        /// Name of the API Specification Collection
        api: String,
        /// Programming language to generate an SDK for
        language: GenerationLanguageClap,
        /// The name of the repository
        repo_name: String,
        /// The semantic version to assign to the SDK
        semver: String,
        #[arg(long, short)]
        /// Output path of generated source files, default: ./
        output: Option<PathBuf>,
    },
    /// **Enterprise Only!**
    /// Update a Sideko managed SDK. This command returns the git patch file to update your SDK to match an updated API
    Update {
        // Path to the existing SDK
        repo_path: PathBuf,
        /// Name of the SDK. Use sdk list to see existing SDKs
        sdk_name: String,
        /// The semantic version to assign to this updated SDK
        semver: String,
    },
    /// **Enterprise Only!**
    /// List all Sideko managed SDKs for an API Specification Collection
    List {
        /// The name of the API in Sideko. e.g. my-rest-api
        api_name: String,
    },
}

#[derive(Debug, Subcommand)]
enum ApiCommands {
    /// List your API Specification Collections
    List {
        /// Pass name to filter by api name to see the versions of a single API e.g. my-rest-api
        #[arg(long, short)]
        name: Option<String>,
    },
    /// Create a new API Specification Collection
    Create {
        /// Either a file path to an OpenAPI yml/json OR a public URL hosting the OpenAPI specification yml/json
        openapi_source: String,
        /// The semantic version to assign to the API
        semver: String,
        /// The name of the API in Sideko. e.g. my-rest-api
        name: String,
        /// Plain text or HTML notes about the new API specification
        #[arg(long)]
        notes: Option<String>,
    },
    /// Upload a new version of a spec to your existing API Specification Collection
    Update {
        /// The name of your API in Sideko. e.g. my-rest-api
        name: String,
        /// Either a file path to an OpenAPI yml/json OR a public URL hosting the OpenAPI specification yml/json
        openapi_source: String,
        /// Either the semantic version OR one of "major" "minor" or "patch" to automatically increment the version from the latest existing version
        #[clap(value_parser = parse_semver_or_increment)]
        semver: SemverOrIncrement,
        /// Plain text or HTML notes about the new API specification
        #[arg(long, short)]
        notes: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum DocCommands {
    /// List your documentation projects
    List {},
    /// Trigger a documentation deployment to preview or production
    Deploy {
        /// The name of the Doc Project in Sideko. e.g. my-rest-api-docs
        name: String,
        #[arg(long)]
        /// Flag to deploy to production environment. If not set, it will deploy to preview
        prod: bool,
        #[arg(long)]
        /// Flag to not poll until the deployment has completed
        no_wait: bool,
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
        Commands::MdDocs { save } => {
            if *save {
                let options = MarkdownOptions::new();
                let options = options.title(
                    "The Sideko Command Line Interface for programmatically generating API tools"
                        .into(),
                );
                let options = options.show_footer(false);
                let options = options.show_table_of_contents(false);
                let docs = clap_markdown::help_markdown_custom::<Cli>(&options);
                let docs_path = std::env::current_dir().unwrap();
                let docs_path = docs_path.join("../docs/CLI.md");
                std::fs::write(docs_path, docs.as_bytes()).expect("could not write docs");
                Ok(())
            } else {
                clap_markdown::print_help_markdown::<Cli>();
                Ok(())
            }
        }
        Commands::Sdk(sdk_command) => {
            match sdk_command {
                SdkCommands::Try {
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
                            result::Error::general("Failed determining cwd for --output default")
                        })?
                    };

                    // Construct cmd input params
                    let params = cmds::sdk::GenerateSdkParams {
                        source: cmds::sdk::OpenApiSource::from(openapi_source),
                        destination,
                        language: language.inner.clone(),
                        base_url: base_url.clone(),
                        package_name: package_name.clone(),
                    };

                    if let OpenApiSource::Raw(_) = params.source {
                        log::error!("Unable to parse OpenAPI as a URL or Path");
                        return Err(result::Error::general(
                            "Unable to parse OpenAPI as a URL or Path",
                        ));
                    };

                    cmds::sdk::handle_try(&params).await
                }
                SdkCommands::Create {
                    api,
                    language,
                    repo_name,
                    semver,
                    output,
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
                    cmds::sdk::handle_create(&language.inner, api, repo_name, semver, &destination)
                        .await
                }
                SdkCommands::List { api_name } => cmds::sdk::handle_list_sdks(api_name).await,
                SdkCommands::Update {
                    repo_path,
                    sdk_name,
                    semver,
                } => cmds::sdk::handle_update(repo_path, sdk_name, semver).await,
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
            ApiCommands::List { name } => cmds::apis::handle_list_apis(name).await,
            ApiCommands::Create {
                openapi_source,
                semver,
                name,
                notes,
            } => {
                let openapi = load_openapi(&cmds::sdk::OpenApiSource::from(openapi_source)).await?;
                cmds::apis::create_new_api_project(
                    &NewApiSpec {
                        version: semver.clone(),
                        openapi,
                        mock_server_enabled: Some(true), // default to turning on the mock server
                        notes: notes.clone(),
                    },
                    name.clone(),
                )
                .await
            }
            ApiCommands::Update {
                name,
                openapi_source,
                semver,
                notes,
            } => {
                let api_versions = data_list_versions(name.clone()).await?;
                let semver = match semver {
                    SemverOrIncrement::Increment(semver_increment) => {
                        let latest_version = find_latest_version(&api_versions);
                        if let Some(latest_version) = latest_version {
                            let mut new_semver = latest_version.clone();
                            match semver_increment {
                                SemverIncrement::Major => {
                                    new_semver.major += 1;
                                    new_semver.minor = 0;
                                    new_semver.patch = 0;
                                }
                                SemverIncrement::Minor => {
                                    new_semver.minor += 1;
                                    new_semver.patch = 0;
                                }
                                SemverIncrement::Patch => {
                                    new_semver.patch += 1;
                                }
                            }
                            new_semver.to_string()
                        } else {
                            return Err(result::Error::general("No API Version to update"));
                        }
                    }
                    SemverOrIncrement::Semver(semver) => {
                        let semver = Version::parse(semver)
                            .map_err(|_| result::Error::general("Invalid semantic version"))?;
                        semver.to_string()
                    }
                };
                cmds::apis::create_new_api_project_version(
                    name.clone(),
                    &NewApiSpec {
                        openapi: load_openapi(&cmds::sdk::OpenApiSource::from(openapi_source))
                            .await?,
                        version: semver.clone(),
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
                name,
                prod,
                no_wait,
            } => cmds::docs::handle_deploy_docs(name, *prod, *no_wait).await,
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

fn find_latest_version(api_versions: &[ApiSpec]) -> Option<Version> {
    api_versions
        .iter()
        .filter_map(|v| {
            semver::Version::parse(&v.version)
                .ok()
                .filter(|parsed_version| parsed_version.pre.is_empty())
        })
        .max()
}

fn parse_semver_or_increment(s: &str) -> Result<SemverOrIncrement, String> {
    SemverOrIncrement::from_str(s)
}
