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

pub fn new_template(name: &str) -> Result<(), ZagreusError> {
    let template_dir = Path::new(name);

    // Return Err if the directory already exists.
    if template_dir.exists() {
        return simple_error(&format!("Directory '{}' already exists", name));
    }

    // Try creating new template directory. After this is successful, we are sure that
    // `template_dir` points to a new directory created by this process. Hence, we can safely delete
    // it and its contents in case we need to roll back after an error.
    fs::create_dir(template_dir)?;

    // Create dirs and files, roll back if there is an error.
    if let Err(error) = create_dirs_and_files(name, template_dir) {
        error!("Unable to create new template, rolling back");
        if let Err(rollback_error) = rollback(template_dir) {
            error!("Unable to roll back: {:?}", rollback_error);
        }
        // Return the original template creation error on any case, rather than rollback error.
        return Err(error);
    }

    Ok(())
}

fn create_dirs_and_files(template_name: &str, template_dir: &Path) -> Result<(), ZagreusError> {
    // Create assets subdirectory.
    fs::create_dir(template_dir.join(ASSETS_FOLDER_NAME))?;

    // Create boilerplate files.
    create_template_config_file(template_name, template_dir)?;
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

fn rollback(template_dir: &Path) -> Result<(), ZagreusError> {
    if !template_dir.exists() {
        // Nothing to roll back, no directory was created yet.
        return Ok(());
    }

    // Remove created files and directories. This requires some duplication path definitions, but
    // it is safer than just using `fs::remove_dir_all(template_dir)`, which could recursively
    // remove the wrong directory.
    remove_inode(&template_dir.join(ASSETS_FOLDER_NAME))?;
    remove_inode(&template_dir.join(TEMPLATE_CONFIG_FILE_NAME))?;
    remove_inode(&template_dir.join(ELEMENT_CONFIG_INPUT_FILE_NAME))?;
    remove_inode(&template_dir.join(ANIMATION_CONFIG_INPUT_FILE_NAME))?;
    remove_inode(template_dir)?;

    Ok(())
}

fn remove_inode(path: &Path) -> Result<(), ZagreusError> {
    if !path.exists() {
        // Nothing to remove here.
        return Ok(());
    }

    // Remove inode.
    if path.is_dir() {
        fs::remove_dir(path)?;
    } else {
        fs::remove_file(path)?;
    }

    Ok(())
}
