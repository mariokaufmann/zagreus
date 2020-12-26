#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use crate::cli::ZagreusSubcommand;
use std::time::Duration;

mod build;
mod cli;
mod data;
mod error;
mod fs;
mod logger;
mod upload;
mod jobs;
mod file_watcher;

const TEMPLATE_CONFIG_FILE_NAME: &str = "zagreus-template.yaml";
const BUILD_FOLDER_NAME: &str = "build";

fn main() {
    let command = cli::get_command();
    logger::init_logger(command.is_verbose());

    loop {
        // TODO: POC only, move this to the build job.
        let _ = file_watcher::wait_for_update(Duration::from_secs(2));
        info!("rebuilding...");
    }

    let result = match command.subcommand() {
        ZagreusSubcommand::New { name } => jobs::new_template(name),
        ZagreusSubcommand::Build { watch, upload } => jobs::build_template(watch, upload),
        ZagreusSubcommand::Upload => jobs::upload_template(),
    };

    match result {
        Ok(()) => info!("Processing complete"),
        Err(error) => error!("Unable to process command: {}", error),
    };
}