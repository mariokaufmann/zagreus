use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;

/// init logger configuration or panic if something fails (since we cannot log yet)
pub fn init_logger() {
    const CONSOLE_LOGGER_NAME: &str = "console_logger";
    let console_appender = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build(CONSOLE_LOGGER_NAME, Box::new(console_appender)))
        .build(Root::builder().appender(CONSOLE_LOGGER_NAME).build(LevelFilter::Info))
        .unwrap_or_else(|err| {
            panic!("Could not construct logging config: {}", err);
        });
    log4rs::init_config(config).unwrap();
}