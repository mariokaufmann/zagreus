#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use crate::cli::ZagreusSubcommand;
use crate::data::TemplateConfig;
use crate::error::{error_with_message, simple_error, ZagreusError};

mod build;
mod cli;
mod data;
mod error;
mod fs;
mod logger;
mod upload;

const TEMPLATE_CONFIG_FILE_NAME: &str = "zagreus-template.yaml";
const BUILD_FOLDER_NAME: &str = "build";

fn main() {
    let command = cli::get_command();
    logger::init_logger(command.is_verbose());

    let result = match command.subcommand() {
        ZagreusSubcommand::New { name } => new_template(name),
        ZagreusSubcommand::Build { watch, upload } => build_template(watch, upload),
        ZagreusSubcommand::Upload => upload_template(),
    };

    match result {
        Ok(()) => info!("Processing complete"),
        Err(error) => error!("Unable to process command: {}", error),
    };
}

fn new_template(_name: String) -> Result<(), ZagreusError> {
    Err(ZagreusError::from(
        "Creating a template is not yet supported.".to_string(),
    ))
}

fn build_template(_watch: bool, upload: bool) -> Result<(), ZagreusError> {
    let template_config = load_template_config()?;
    let build_dir = PathBuf::from(BUILD_FOLDER_NAME);

    if let Err(error) = build::build_template(build_dir.as_path(), &template_config) {
        return error_with_message(
            &format!("Could not build template {}", &template_config.name),
            error,
        );
    }

    info!("Successfully built template '{}'", template_config.name);

    if upload {
        return upload_template();
    }

    Ok(())
}

fn upload_template() -> Result<(), ZagreusError> {
    let template_config = load_template_config()?;
    let zipped_template_path = get_zipped_template_path()?;

    let server_url = format!(
        "{}:{}",
        &template_config.dev_server.address, &template_config.dev_server.port
    );

    match upload::TemplateUploader::new(&server_url, &template_config.name, &zipped_template_path) {
        Ok(template_uploader) => {
            if let Err(err) = template_uploader.upload_template() {
                return error_with_message("Could not upload template", err);
            }
        }
        Err(error) => {
            return error_with_message("Could not construct template uploader", error);
        }
    }

    info!(
        "Successfully uploaded template '{}' to {}",
        template_config.name, server_url
    );
    Ok(())
}

fn load_template_config() -> Result<TemplateConfig, ZagreusError> {
    let file_path = PathBuf::from(TEMPLATE_CONFIG_FILE_NAME);
    crate::data::load_config::<TemplateConfig>(&file_path)
}

fn get_zipped_template_path() -> Result<PathBuf, ZagreusError> {
    let build_dir = PathBuf::from(BUILD_FOLDER_NAME);
    if !build_dir.exists() {
        return simple_error("Build directory not found. Did you build the template?");
    }

    let zipped_template_path = build::get_zipped_template_file_path(build_dir.as_path());
    if !zipped_template_path.exists() {
        return simple_error(
            "Zipped template not found in build directory. Try rebuilding the template.",
        );
    }

    Ok(zipped_template_path)
}
