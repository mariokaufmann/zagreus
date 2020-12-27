use crate::data::TemplateConfig;
use crate::error::{error_with_message, simple_error, ZagreusError};
use crate::file_watcher;
use crate::{build, upload, BUILD_FOLDER_NAME, TEMPLATE_CONFIG_FILE_NAME};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn new_template(_name: String) -> Result<(), ZagreusError> {
    simple_error("Creating a template is not yet supported.")
}

pub fn build_template(watch: bool, upload: bool) -> Result<(), ZagreusError> {
    let template_config = load_template_config()?;
    let build_dir = Path::new(BUILD_FOLDER_NAME);

    if !watch {
        return build_once(&template_config, build_dir, upload);
    };

    info!("Watch mode started");
    let file_watcher_handle = file_watcher::listen()?;
    loop {
        if let Err(err) = build_once(&template_config, build_dir, upload) {
            error!("{:?}", err);
        }
        file_watcher::wait_for_update(&file_watcher_handle, Duration::from_secs(1));
    }
}

fn build_once(
    template_config: &TemplateConfig,
    build_dir: &Path,
    upload: bool,
) -> Result<(), ZagreusError> {
    if let Err(error) = build::build_template(build_dir, &template_config) {
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
