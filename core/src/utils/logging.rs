pub fn init_logger(quiet: bool, verbose: bool) {
    let level = if quiet {
        log::Level::Error
    } else if verbose {
        log::Level::Debug
    } else {
        log::Level::Info
    };

    let _ = if level == log::Level::Trace {
        env_logger::Builder::new().try_init()
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
