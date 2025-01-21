use std::env;

use crate::{
    cmds,
    result::CliResult,
    styles::{self, fmt_cyan},
    utils,
};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser)]
#[command(name = "sideko")]
#[command(author = "Team Sideko <team@sideko.dev>")]
#[command(about = &fmt_cyan(r#"
```
.*....*......*.....*......*....*........*....*.....

..####...######..#####...######..##..##...####..
.##........##....##..##..##......##.##...##..##.
..####.....##....##..##..####....####....##..##.
.....##....##....##..##..##......##.##...##..##.
..####...######..#####...######..##..##...####..
................................................

- Your API Ecosystem, On Autopilot
*....*......*.....*......*.....*......*.....*.....*            
```                                                                                                                             
"#), long_about = None)]
#[command(version)]
pub struct SidekoCli {
    #[command(subcommand)]
    command: SidekoCommands,

    /// No logging except for errors
    #[arg(long, short = 'q', global = true)]
    quiet: bool,

    /// Verbose logging (-v) or trace logging (-vv)
    #[arg(long, short = 'v', action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Load config from custom path
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

        utils::check_for_updates().await?;

        // Run command
        match &self.command {
            SidekoCommands::Login(cmd) => cmd.handle().await,
            SidekoCommands::Logout(cmd) => cmd.handle().await,
            SidekoCommands::Api(cmd) => cmd.handle().await,
            SidekoCommands::Sdk(cmd) => cmd.handle().await,
            SidekoCommands::Doc(cmd) => cmd.handle().await,
            SidekoCommands::Config(cmd) => cmd.handle().await,
        }
    }
}

#[derive(Subcommand)]
#[command(styles=styles::get_styles())]
enum SidekoCommands {
    /// Authenticate CLI interactively via browser
    Login(cmds::LoginCommand),

    /// Manage API specifications
    #[command(subcommand)]
    Api(cmds::ApiSubcommand),

    /// Generate, customize, and sync SDKs
    #[command(subcommand)]
    Sdk(cmds::SdkSubcommand),

    /// Manage API documentation websites
    #[command(subcommand)]
    Doc(cmds::DocSubcommand),

    /// Logout of Sideko
    ///
    /// Removes the Sideko API key from the OS-native key service
    /// (e.g. `keychain` on macOS, `keyutils` on Linux, `Windows Credential Manager` on Windows)
    Logout(cmds::LogoutCommand),

    /// Configure the CLI
    #[command(subcommand)]
    Config(cmds::ConfigSubcommand),
}

pub async fn cli(args: Vec<String>) -> CliResult<()> {
    let cli = SidekoCli::parse_from(args);

    let handled = cli.handle().await;
    if let Err(e) = &handled {
        e.log();
        info!("Re-run the command in verbose mode (-v/-vv) to for more information")
    }

    handled
}
