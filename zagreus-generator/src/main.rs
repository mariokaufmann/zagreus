#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use crate::cli::ZagreusSubcommand;

mod build;
mod cli;
mod data;
mod error;
mod fs;
mod logger;
mod upload;
mod jobs;

const TEMPLATE_CONFIG_FILE_NAME: &str = "zagreus-template.yaml";
const BUILD_FOLDER_NAME: &str = "build";

fn main() {
    let command = cli::get_command();
    logger::init_logger(command.is_verbose());

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