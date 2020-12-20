#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::data::TemplateConfig;

mod build;
mod data;
mod error;
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
    let command: ZagreusCommand = ZagreusCommand::from_args();
    logger::init_logger(command.debug);

    match command.subcommand {
        Subcommand::NewCommand { name } => new_template(name),
        Subcommand::BuildCommand {
            path,
            watch,
            upload,
        } => build(path, watch, upload),
        Subcommand::UploadCommand { path } => upload(path),
    }
}

fn new_template(name: String) {
    trace!("Creating new template at {:?}", name);
}

fn build(path: PathBuf, watch: bool, upload: bool) {
    trace!(
        "Building {:?}, watch={:?}, upload={:?}",
        path,
        watch,
        upload
    );
}

fn upload(path: PathBuf) {
    trace!("Uploading to {:?}", path);
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Zagreus Template Generator",
    about = "CLI application for generating, building and uploading Zagreus templates."
)]
struct ZagreusCommand {
    #[structopt(short, long, help = "Enables debug and trace logging")]
    debug: bool,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "new", about = "Generates a new boilerplate template.")]
    NewCommand {
        #[structopt(help = "Name of the new template")]
        name: String,
    },

    #[structopt(name = "build", about = "Builds a template.")]
    BuildCommand {
        #[structopt(parse(from_os_str), help = "Path to the template to be built")]
        path: PathBuf,

        #[structopt(short, long, help = "Keep running and rebuild on file changes")]
        watch: bool,

        #[structopt(
            short,
            long,
            help = "Upload template to the configured Zagreus server after every build"
        )]
        upload: bool,
    },

    #[structopt(
        name = "upload",
        about = "Uploads a template to the Zagreus server configured in the template."
    )]
    UploadCommand {
        #[structopt(parse(from_os_str), help = "Path to the template to be uploaded")]
        path: PathBuf,
    },
}
