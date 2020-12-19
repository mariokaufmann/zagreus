use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

/// init logger configuration or panic if something fails (since we cannot log yet)
pub fn init_logger(is_debug: bool) {
    const CONSOLE_LOGGER_NAME: &str = "console_logger";
    let console_appender = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build(CONSOLE_LOGGER_NAME, Box::new(console_appender)))
        .build(
            Root::builder()
                .appender(CONSOLE_LOGGER_NAME)
                .build(level_filter(is_debug)),
        )
        .unwrap_or_else(|err| {
            panic!("Could not construct logging config: {}", err);
        });
    log4rs::init_config(config).unwrap();
}

fn level_filter(is_debug: bool) -> LevelFilter {
    if is_debug {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    }
}
