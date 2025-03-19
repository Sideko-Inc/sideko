use std::env;

use crate::{
    cmds,
    result::CliResult,
    styles::{self, fmt_cyan},
    utils,
};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use log::{error, info, warn};
use sideko_rest_api::models::CliUpdateSeverityEnum;

#[derive(Parser)]
#[command(name = "sideko")]
#[command(author = "team sideko <team@sideko.dev>")]
#[command(about = &fmt_cyan(r#"
```       _      _        _          
     ___ (_)  __| |  ___ | | __  ___  
    / __|| | / _` | / _ \| |/ / / _ \ 
    \__ \| || (_| ||  __/|   < | (_) |
    |___/|_| \__,_| \___||_|\_\ \___/ 

    your api ecosystem on autopilot
```                                                                                                                            
"#), long_about = None)]
#[command(version)]
pub struct SidekoCli {
    #[command(subcommand)]
    command: SidekoCommands,

    /// no logging except for errors
    #[arg(long, short = 'q', global = true)]
    quiet: bool,

    /// verbose logging (-v) or trace logging (-vv)
    #[arg(long, short = 'v', action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// load config from custom path
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file
    )]
    config: Option<Utf8PathBuf>,
}
impl SidekoCli {
    async fn handle(&self) -> CliResult<()> {
        // init logger and environment
        utils::logging::init_logger(self.quiet, self.verbose);

        if let Some(cfg_path) = &self.config {
            env::set_var(utils::config::ConfigKey::ConfigPath.to_string(), cfg_path);
        }
        utils::config::load()?;

        let updates = utils::check_for_updates().await?;

        // Run command
        let cmd_res = match &self.command {
            SidekoCommands::Login(cmd) => cmd.handle().await,
            SidekoCommands::Logout(cmd) => cmd.handle().await,
            SidekoCommands::Api(cmd) => cmd.handle().await,
            SidekoCommands::Account(cmd) => cmd.handle().await,
            SidekoCommands::Sdk(cmd) => cmd.handle().await,
            SidekoCommands::Doc(cmd) => cmd.handle().await,
            SidekoCommands::Config(cmd) => cmd.handle().await,
        };

        // log update notices
        for update in updates {
            match update.severity {
                CliUpdateSeverityEnum::Info => {
                    info!("{}", update.message);
                }
                CliUpdateSeverityEnum::Suggested => {
                    warn!("{}", update.message);
                }
                CliUpdateSeverityEnum::Required => {
                    error!("{}", update.message);
                }
            }
        }

        cmd_res
    }
}

#[derive(Subcommand)]
#[command(styles=styles::get_styles())]
enum SidekoCommands {
    /// authenticate cli interactively via browser
    Login(cmds::LoginCommand),

    /// manage api specifications
    #[command(subcommand)]
    Api(cmds::ApiSubcommand),

    /// manage your sideko account
    #[command(subcommand)]
    Account(cmds::AccountSubcommand),

    /// generate, customize, and sync sdks
    #[command(subcommand)]
    Sdk(cmds::SdkSubcommand),

    /// manage api documentation websites
    #[command(subcommand)]
    Doc(cmds::DocSubcommand),

    /// logout of sideko
    ///
    /// removes the api key from the os-native key service
    /// (e.g. `keychain` on macos, `keyutils` on linux,  or `windows credential manager`)
    Logout(cmds::LogoutCommand),

    /// configure the cli
    #[command(subcommand)]
    Config(cmds::ConfigSubcommand),
}

pub async fn cli(args: Vec<String>) -> CliResult<()> {
    let cli = SidekoCli::parse_from(args);

    let handled = cli.handle().await;
    if let Err(e) = &handled {
        e.log();
        info!("re-run the command in verbose mode (-v/-vv) to for more information")
    }

    handled
}
