use log::{info, Level};
use std::io::Write;
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

    let mut builder = env_logger::builder();

    if level == log::Level::Trace {
        builder.filter_level(level.to_level_filter());
    } else if level == log::Level::Debug {
        builder.filter_module("sideko", level.to_level_filter());
    } else {
        // info, warn, error
        builder
            .filter_module("sideko", level.to_level_filter())
            .format(|buf, record| {
                if record.level() == Level::Info {
                    writeln!(buf, "{}", record.args())
                } else {
                    let log_style = buf.default_level_style(record.level());
                    writeln!(
                        buf,
                        "{log_style}[{}]{log_style:#} {}",
                        record.level(),
                        record.args()
                    )
                }
            });
    }

    let _ = builder.try_init();
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
    info!("\n{table}\n");
}
