#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::Path;

use crate::data::TemplateConfig;

mod build;
mod data;
mod error;
mod logger;
mod upload;

const TEMPLATE_CONFIG_FILE_NAME: &str = "zagreus-template.yaml";
const BUILD_FOLDER_NAME: &str = "build";

fn main() {
    logger::init_logger();

    let template_config = crate::data::load_config::<TemplateConfig>(Path::new(TEMPLATE_CONFIG_FILE_NAME)).unwrap();

    let build_folder = Path::new(BUILD_FOLDER_NAME);

    if let Err(err) = build::build_template(build_folder, &template_config) {
        error!("Could not build template {}: {}", &template_config.name, err);
        return;
    }

    let zipped_template_path = build::get_zipped_template_file_path(build_folder);
    match upload::TemplateUploader::new(&format!("{}:{}", &template_config.dev_server.address,
                                                 &template_config.dev_server.port), &template_config.name,
                                        &zipped_template_path) {
        Ok(template_uploader) => {
            if let Err(err) = template_uploader.upload_template() {
                error!("Could not upload template: {}.", err);
            }
        }
        Err(err) => error!("Could not construct template uploader: {}.", err),
    }

    info!("Finished processing.");
}


