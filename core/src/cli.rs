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
    #[arg(
        long,
        short = 'q',
        global = true,
        help = "No logging except for errors"
    )]
    quiet: bool,
    #[arg(long, short = 'v', global = true, help = "Verbose logging")]
    verbose: bool,
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file,
        help = "Load config from custom path"
    )]
    config: Option<Utf8PathBuf>,
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

    // init logger and environment
    utils::logging::init_logger(cli.quiet, cli.verbose);

    if let Some(cfg_path) = &cli.config {
        env::set_var(utils::config::ConfigKey::ConfigPath.to_string(), cfg_path);
    }
    utils::config::load()?;

    utils::check_for_updates().await?;

    // Run command
    let cmd_res = match cli.command {
        SidekoCommands::Login(cmd) => cmd.handle().await,
        SidekoCommands::Api(cmd) => cmd.handle().await,
    };

    if let Err(e) = &cmd_res {
        e.log();
    }

    cmd_res
}
