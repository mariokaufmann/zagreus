use crate::build::{
    ANIMATION_CONFIG_INPUT_FILE_NAME, BUILD_FOLDER_NAME, ELEMENT_CONFIG_INPUT_FILE_NAME,
    INPUT_SVG_FILE_NAME,
};
use crate::data::TemplateConfig;
use crate::error::{error_with_message, simple_error, ZagreusError};
use crate::file_watcher;
use crate::{build, new, upload, TEMPLATE_CONFIG_FILE_NAME};
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

const FILE_WATCHER_DEBOUNCE_DELAY: u64 = 200;

pub fn new_template(name: String) -> Result<(), ZagreusError> {
    new::new_template(&name)
}

pub fn build_template(watch: bool, upload: bool) -> Result<(), ZagreusError> {
    verify_required_files_present()?;

    let template_config = load_template_config()?;
    let build_dir = Path::new(BUILD_FOLDER_NAME);

    if !watch {
        return build_once(&template_config, build_dir, upload);
    };

    info!("Watch mode started");
    let file_watcher_rx = file_watcher::spawn(
        env::current_dir()?,
        Duration::from_millis(FILE_WATCHER_DEBOUNCE_DELAY),
    )?;
    loop {
        // Build the template.
        if let Err(error) = build_once(&template_config, build_dir, upload) {
            // If a build error occurs, log the error and wait for the next file change.
            error!("{:?}", error);
        }

        // Wait for a file change.
        file_watcher_rx.recv()?;

        // Wait for further file changes if necessary, until all the required files are present.
        while let Err(error) = verify_required_files_present() {
            error!("{:?}", error);
            file_watcher_rx.recv()?;
        }
    }
}

/// Checks whether all the files required for building the template are present. Logs an error for
/// each missing file. Returns an error if at least one file is missing, `Ok` else.
fn verify_required_files_present() -> Result<(), ZagreusError> {
    let required_files = [
        TEMPLATE_CONFIG_FILE_NAME,
        ELEMENT_CONFIG_INPUT_FILE_NAME,
        ANIMATION_CONFIG_INPUT_FILE_NAME,
        INPUT_SVG_FILE_NAME,
    ];
    match required_files
        .iter()
        .map(Path::new)
        .filter(|path| !path.exists())
        .inspect(|missing_path| error!("Missing required file: {:?}", missing_path))
        .count()
    {
        0 => Ok(()),
        1 => simple_error("Unable to build template, missing a required input file"),
        _ => simple_error("Unable to build template, missing multiple required input files"),
    }
}

fn build_once(
    template_config: &TemplateConfig,
    build_dir: &Path,
    upload: bool,
) -> Result<(), ZagreusError> {
    info!("Building template {}...", &template_config.name);
    if let Err(error) = build::build_template(build_dir, template_config) {
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

pub fn upload_template() -> Result<(), ZagreusError> {
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
    let file_path = Path::new(TEMPLATE_CONFIG_FILE_NAME);
    crate::data::load_config::<TemplateConfig>(file_path)
}

fn get_zipped_template_path() -> Result<PathBuf, ZagreusError> {
    let build_dir = Path::new(BUILD_FOLDER_NAME);
    if !build_dir.exists() {
        return simple_error("Build directory not found. Did you build the template?");
    }

    let zipped_template_path = build::get_zipped_template_file_path(build_dir);
    if !zipped_template_path.exists() {
        return simple_error(
            "Zipped template not found in build directory. Try rebuilding the template.",
        );
    }

    Ok(zipped_template_path)
}
