use crate::build::ASSETS_FOLDER_NAME;
use crate::data::{DevServerConfig, OnLoadConfig, TemplateConfig};
use crate::error::{simple_error, ZagreusError};
use crate::TEMPLATE_CONFIG_FILE_NAME;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DEFAULT_DEV_SERVER_ADDRESS: &str = "localhost";
const DEFAULT_DEV_SERVER_PORT: u16 = 58179;

pub fn new_template(name: &str) -> Result<(), ZagreusError> {
    let template_dir = Path::new(name);

    // Return Err if the directory already exists.
    if template_dir.exists() {
        return simple_error(&format!("Directory '{}' already exists", name));
    }

    // Create template directory and assets subdirectory.
    fs::create_dir(template_dir)?;
    fs::create_dir(template_dir.join(ASSETS_FOLDER_NAME))?;

    create_template_config_file(name, template_dir)?;

    Ok(())

    /*
    TODO:
     - [ok] create a subdirectory with the given name
     - in the subdir, create:
       - [ok] template config with subdir name as a template name
       - empty animation configs
       - empty elements configs (see PR #13)
       - [ok] empty asset folder in the subdirectory
     */
}

fn create_template_config_file(
    template_name: &str,
    template_dir: &Path,
) -> Result<(), ZagreusError> {
    let on_load_config = OnLoadConfig {
        animation_sequences: vec![],
    };
    let dev_server_config = DevServerConfig {
        address: String::from(DEFAULT_DEV_SERVER_ADDRESS),
        port: DEFAULT_DEV_SERVER_PORT,
    };
    let template_config = TemplateConfig {
        name: String::from(template_name),
        on_load: on_load_config,
        dev_server: dev_server_config,
    };

    let serialized = serde_yaml::to_string(&template_config)?;
    write_to_new_file(&template_dir.join(TEMPLATE_CONFIG_FILE_NAME), &serialized)?;

    Ok(())
}

fn write_to_new_file(file_path: &Path, content: &str) -> Result<(), ZagreusError> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
