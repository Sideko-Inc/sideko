use std::env;

use crate::{cmds, result::CliResult, styles, utils};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sideko")]
#[command(author = "Team Sideko <team@sideko.dev>")]
#[command(about = "Start generating tools for your APIs wit Sideko!", long_about = None)]
#[command(version)]
struct SidekoCli {
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
            SidekoCommands::Api(cmd) => cmd.handle().await,
        }
    }
}

#[derive(Subcommand)]
#[command(styles=styles::get_styles())]
enum SidekoCommands {
    /// Authenticate the CLI interactively via the browser
    Login(cmds::LoginCommand),
    /// Command group to manage your APIs
    #[command(subcommand)]
    Api(cmds::ApiSubcommand),
}

pub async fn cli(args: Vec<String>) -> CliResult<()> {
    let cli = SidekoCli::parse_from(args);

    let handled = cli.handle().await;
    if let Err(e) = &handled {
        e.log();
    }

    handled
}
