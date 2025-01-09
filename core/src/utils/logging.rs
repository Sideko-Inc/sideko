use log::info;
use tabled::settings::{peaker::Priority, Width};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

pub fn init_logger(quiet: bool, verbose: u8) {
    let level = if quiet {
        log::Level::Error
    } else if verbose == 0 {
        log::Level::Info
    } else if verbose == 1 {
        log::Level::Debug
    } else {
        log::Level::Trace
    };

    let _ = if level == log::Level::Trace {
        env_logger::Builder::new()
            .filter_level(level.to_level_filter())
            .try_init()
    } else if level > log::Level::Info {
        env_logger::Builder::new()
            .filter_module("sideko", level.to_level_filter())
            .try_init()
    } else {
        env_logger::Builder::new()
            .filter_module("sideko", level.to_level_filter())
            .format_target(false)
            .format_timestamp(None)
            .try_init()
    };
}

pub fn log_json_raw<T: ?Sized + serde::Serialize>(val: &T) {
    info!(
        "{}",
        serde_json::to_string_pretty(val).unwrap_or_else(|_| serde_json::json!(val).to_string())
    )
}

pub fn log_table(mut table: tabled::Table) {
    // consistent table format that fits in existing terminal size
    table.with(tabled::settings::Style::modern());

    if let Some((TerminalWidth(width), TerminalHeight(_height))) = terminal_size() {
        table.with(Width::wrap(width as usize).priority(Priority::max(true)));
    }

    // TODO: using `info!` here adds \n to any newlines meaning the terminal sizing is all off

    println!("\n{table}\n");
}
