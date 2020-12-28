use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::APPLICATION_NAME;

const LOG_FILE_NAME: &str = "log.log";
const ROTATED_LOG_FILE_NAME: &str = "log.{}.log";
const MAX_LOG_SIZE_BYTES: u64 = 5_000_000;
const LOG_FILE_COUNT: u32 = 5;

/// init logger configuration or panic if something fails (since we cannot log yet)
pub fn init_logger() {
    let log_folder_path = crate::fs::get_log_folder_path(APPLICATION_NAME).unwrap_or_else(|err| {
        panic!("Could not get log file path: {}", err);
    });
    let mut log_file_path = log_folder_path.clone();
    log_file_path.push(LOG_FILE_NAME);

    let mut rotated_log_file_path = log_folder_path;
    rotated_log_file_path.push(ROTATED_LOG_FILE_NAME);
    let rotated_log_file_path = rotated_log_file_path.to_str().unwrap();

    const FILE_LOGGER_NAME: &str = "file_logger";
    const CONSOLE_LOGGER_NAME: &str = "console_logger";
    let roller = FixedWindowRoller::builder()
        .build(rotated_log_file_path, LOG_FILE_COUNT)
        .unwrap_or_else(|err| {
            panic!("Could not setup fixed window roller: {}.", err);
        });
    let rolling_file_policy = CompoundPolicy::new(
        Box::new(SizeTrigger::new(MAX_LOG_SIZE_BYTES)),
        Box::new(roller),
    );
    let file_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} [{h({l})}] - {T} - {m}{n}",
        )))
        .build(log_file_path, Box::new(rolling_file_policy))
        .unwrap();

    let console_appender = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build(FILE_LOGGER_NAME, Box::new(file_appender)))
        .appender(Appender::builder().build(CONSOLE_LOGGER_NAME, Box::new(console_appender)))
        .build(
            Root::builder()
                .appender(CONSOLE_LOGGER_NAME)
                .appender(FILE_LOGGER_NAME)
                .build(LevelFilter::Info),
        )
        .unwrap_or_else(|err| {
            panic!("Could not construct logging config: {}", err);
        });
    log4rs::init_config(config).unwrap();
}
