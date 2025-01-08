use crate::{cmds, result::CliResult, styles, utils};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sideko")]
#[command(author = "Team Sideko <team@sideko.dev>")]
#[command(about = "Start generating tools for your APIs wit Sideko!", long_about = None)]
#[command(version)]
#[command(propagate_version = true)]
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
}

#[derive(Subcommand)]
#[command(styles=styles::get_styles())]
enum SidekoCommands {
    /// Authenticate the CLI interactively via the browser
    Login(cmds::LoginCommand),
}

pub async fn cli(args: Vec<String>) -> CliResult<()> {
    let cli = SidekoCli::parse_from(args);

    // init logger and environment
    utils::logging::init_logger(cli.quiet, cli.verbose);
    utils::config::load()?;
    let cmd_res = match cli.command {
        SidekoCommands::Login(cmd) => cmd.handle().await,
    };

    if let Err(e) = &cmd_res {
        e.log();
    }

    cmd_res
}
