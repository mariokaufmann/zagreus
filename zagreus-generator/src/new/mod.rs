use crate::build::{
    ANIMATION_CONFIG_INPUT_FILE_NAME, ASSETS_FOLDER_NAME, ELEMENT_CONFIG_INPUT_FILE_NAME,
};
use crate::data::animation::config::AnimationConfig;
use crate::data::element::ElementsConfig;
use crate::data::TemplateConfig;
use crate::error::{simple_error, ZagreusError};
use crate::TEMPLATE_CONFIG_FILE_NAME;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// TODO: Factor content out into new function, rollback on Err
pub fn new_template(name: &str) -> Result<(), ZagreusError> {
    let template_dir = Path::new(name);

    // Return Err if the directory already exists.
    if template_dir.exists() {
        return simple_error(&format!("Directory '{}' already exists", name));
    }

    // Create template directory and assets subdirectory.
    fs::create_dir(template_dir)?;
    fs::create_dir(template_dir.join(ASSETS_FOLDER_NAME))?;

    // Create boilerplate files.
    create_template_config_file(name, template_dir)?;
    create_animation_config_file(template_dir)?;
    create_element_config_file(template_dir)?;

    Ok(())
}

fn create_template_config_file(
    template_name: &str,
    template_dir: &Path,
) -> Result<(), ZagreusError> {
    let template_config = TemplateConfig::default_with_name(template_name);
    let serialized = serde_yaml::to_string(&template_config)?;
    write_to_new_file(&template_dir.join(TEMPLATE_CONFIG_FILE_NAME), &serialized)?;
    Ok(())
}

fn create_element_config_file(template_dir: &Path) -> Result<(), ZagreusError> {
    let element_config: ElementsConfig = Default::default();
    let serialized = serde_yaml::to_string(&element_config)?;
    write_to_new_file(
        &template_dir.join(ELEMENT_CONFIG_INPUT_FILE_NAME),
        &serialized,
    )?;
    Ok(())
}

fn create_animation_config_file(template_dir: &Path) -> Result<(), ZagreusError> {
    let animation_config: AnimationConfig = Default::default();
    let serialized = serde_yaml::to_string(&animation_config)?;
    write_to_new_file(
        &template_dir.join(ANIMATION_CONFIG_INPUT_FILE_NAME),
        &serialized,
    )?;
    Ok(())
}

fn write_to_new_file(file_path: &Path, content: &str) -> Result<(), ZagreusError> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}