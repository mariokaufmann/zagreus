#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use crate::cli::ZagreusSubcommand;
use crate::data::TemplateConfig;
use crate::error::ZagreusError;

mod build;
mod cli;
mod data;
mod error;
mod fs;
mod logger;
mod upload;

const TEMPLATE_CONFIG_FILE_NAME: &str = "zagreus-template.yaml";
const BUILD_FOLDER_NAME: &str = "build";

#[allow(dead_code)]
fn build_and_upload() {}

fn main() {
    let command = cli::get_command();
    logger::init_logger(command.debug_enabled());

    let result = match command.subcommand() {
        ZagreusSubcommand::New { name } => new_template(name),
        ZagreusSubcommand::Build { watch, upload } => build_template(watch, upload),
        ZagreusSubcommand::Upload => upload_template(),
    };

    if let Err(error) = result {
        error!("Unable to process command: {}", error);
    }
}

fn new_template(name: String) -> Result<(), ZagreusError> {
    trace!("Creating new template '{}'", name);
    Ok(())
}

fn build_template(watch: bool, do_upload: bool) -> Result<(), ZagreusError> {
    trace!(
        "Building template, watch={:?}, upload={:?}",
        watch,
        do_upload
    );

    let template_config = load_template_config()?;

    let build_dir = PathBuf::from(BUILD_FOLDER_NAME);

    if let Err(err) = build::build_template(build_dir.as_path(), &template_config) {
        let error_msg = format!(
            "Could not build template {}: {}",
            &template_config.name, err
        );
        return Err(ZagreusError::from(error_msg));
    }

    build::get_zipped_template_file_path(build_dir.as_path());

    if do_upload {
        return upload_template();
    }

    Ok(())
}

fn upload_template() -> Result<(), ZagreusError> {
    trace!("Uploading template to configured server");

    let build_dir = PathBuf::from(BUILD_FOLDER_NAME);
    let template_config = load_template_config()?;
    let zipped_template_path = build::get_zipped_template_file_path(build_dir.as_path());

    if !zipped_template_path.exists() {
        let error_msg = format!(
            "Zipped template not found in build dir: {:?}",
            zipped_template_path
        );
        return Err(ZagreusError::from(error_msg));
    }

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
    Ok(())
}

fn load_template_config() -> Result<TemplateConfig, ZagreusError> {
    let file_path = PathBuf::from(TEMPLATE_CONFIG_FILE_NAME);
    crate::data::load_config::<TemplateConfig>(&file_path)
}
