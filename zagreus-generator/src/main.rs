#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

use crate::data::TemplateConfig;
use crate::cli::ZagreusSubcommand;

mod cli;
mod build;
mod data;
mod error;
mod fs;
mod logger;
mod upload;

const TEMPLATE_CONFIG_FILE_NAME: &str = "zagreus-template.yaml";
const BUILD_FOLDER_NAME: &str = "build";

#[allow(dead_code)]
fn build_and_upload() {
    let template_config =
        crate::data::load_config::<TemplateConfig>(Path::new(TEMPLATE_CONFIG_FILE_NAME)).unwrap();

    let build_folder = Path::new(BUILD_FOLDER_NAME);

    if let Err(err) = build::build_template(build_folder, &template_config) {
        error!(
            "Could not build template {}: {}",
            &template_config.name, err
        );
        return;
    }

    let zipped_template_path = build::get_zipped_template_file_path(build_folder);
    match upload::TemplateUploader::new(
        &format!(
            "{}:{}",
            &template_config.dev_server.address, &template_config.dev_server.port
        ),
        &template_config.name,
        &zipped_template_path,
    ) {
        Ok(template_uploader) => {
            if let Err(err) = template_uploader.upload_template() {
                error!("Could not upload template: {}.", err);
            }
        }
        Err(err) => error!("Could not construct template uploader: {}.", err),
    }

    info!("Finished processing.");
}

fn main() {
    let command = cli::get_command();
    logger::init_logger(command.debug_enabled());

    match command.subcommand() {
        ZagreusSubcommand::New { name } => new_template(name),
        ZagreusSubcommand::Build {
            path,
            watch,
            upload,
        } => build(path, watch, upload),
        ZagreusSubcommand::Upload { path } => upload(path),
    }
}

fn new_template(name: String) {
    trace!("Creating new template '{}'", name);
}

fn build(path: PathBuf, watch: bool, upload: bool) {
    trace!(
        "Building template {:?}, watch={:?}, upload={:?}",
        path,
        watch,
        upload
    );
}

fn upload(path: PathBuf) {
    trace!("Uploading template {:?} to configured server", path);
}


