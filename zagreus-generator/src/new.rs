use crate::build::{
    ANIMATION_CONFIG_INPUT_FILE_NAME, ASSETS_FOLDER_NAME, ELEMENT_CONFIG_INPUT_FILE_NAME,
};
use crate::data::animation::config::AnimationConfig;
use crate::data::element::ElementsConfig;
use crate::data::TemplateConfig;
use crate::error::{simple_error, ZagreusError};
use crate::TEMPLATE_CONFIG_FILE_NAME;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Creates a new boilerplate template with the given name.
///
/// Creates a new directory with the given name in the current working directory. In there,
/// creates an empty assets directory, as well as skeletons for all the YAML config files required
/// to build a template. Users may still need to provide additional files until the template can be
/// built.
pub fn new_template(name: &str) -> Result<(), ZagreusError> {
    validate_template_name(name)?;

    // Return Err if the directory already exists.
    let template_dir = Path::new(name);
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

/// Checks whether the given template name only contains valid characters. The set of valid
/// characters is defined by this function. Returns an error the name is invalid, with an error
/// message containing the complete set of offending characters.
fn validate_template_name(template_name: &str) -> Result<(), ZagreusError> {
    let allowed_special_chars = ['-'];
    let illegal_chars: HashSet<char> = template_name
        .chars()
        .filter(|char| !(char.is_ascii_alphanumeric() || allowed_special_chars.contains(char)))
        .collect();
    match illegal_chars.is_empty() {
        true => Ok(()),
        false => simple_error(&format!(
            "Template name must only contain characters in [a-zA-Z0-9-], provided name contains: {:?}",
            illegal_chars
        )),
    }
}

/// Creates all directories and boilerplate files required for a new template, not including the
/// new template directory itself. Returns an error if any of them cannot be created.
fn create_dirs_and_files(template_name: &str, template_dir: &Path) -> Result<(), ZagreusError> {
    // Create assets subdirectory.
    fs::create_dir(template_dir.join(ASSETS_FOLDER_NAME))?;

    // Create boilerplate files.
    create_template_config_file(template_name, template_dir)?;
    create_animation_config_file(template_dir)?;
    create_element_config_file(template_dir)?;

    Ok(())
}

/// Creates the default template config file in the new template directory, populates the `name`
/// field with the given template name.
fn create_template_config_file(
    template_name: &str,
    template_dir: &Path,
) -> Result<(), ZagreusError> {
    let template_config = TemplateConfig::default_with_name(template_name);
    let serialized = serde_yaml::to_string(&template_config)?;
    write_to_new_file(&template_dir.join(TEMPLATE_CONFIG_FILE_NAME), &serialized)?;
    Ok(())
}

/// Creates the default element config file in the new template directory.
fn create_element_config_file(template_dir: &Path) -> Result<(), ZagreusError> {
    let element_config: ElementsConfig = Default::default();
    let serialized = serde_yaml::to_string(&element_config)?;
    write_to_new_file(
        &template_dir.join(ELEMENT_CONFIG_INPUT_FILE_NAME),
        &serialized,
    )?;
    Ok(())
}

/// Creates the default animation config file in the new template directory.
fn create_animation_config_file(template_dir: &Path) -> Result<(), ZagreusError> {
    let animation_config: AnimationConfig = Default::default();
    let serialized = serde_yaml::to_string(&animation_config)?;
    write_to_new_file(
        &template_dir.join(ANIMATION_CONFIG_INPUT_FILE_NAME),
        &serialized,
    )?;
    Ok(())
}

/// Creates a new file at the given path and writes the given content to that file. Returns an
/// error if the file already exists, or if an IO error occurs.
fn write_to_new_file(file_path: &Path, content: &str) -> Result<(), ZagreusError> {
    if file_path.exists() {
        // Reaching here is considered a bug: a new (i.e. empty) template directory should be
        // created first, and no file should be created more than once.
        return simple_error(&format!("File already exists: {:?}", file_path));
    }
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Deletes the assets directory, template config file, element config file, animation config file,
/// and newly created template directory. Returns an error if any of these cannot be removed.
fn rollback(template_dir: &Path) -> Result<(), ZagreusError> {
    if !template_dir.exists() {
        // Nothing to roll back, no directory was created yet.
        return Ok(());
    }

    // Remove created files and directories. This requires some duplicate path definitions, but
    // it is safer than just using `fs::remove_dir_all(template_dir)`, which could recursively
    // remove the wrong directory.
    remove_file_or_directory(&template_dir.join(ASSETS_FOLDER_NAME))?;
    remove_file_or_directory(&template_dir.join(TEMPLATE_CONFIG_FILE_NAME))?;
    remove_file_or_directory(&template_dir.join(ELEMENT_CONFIG_INPUT_FILE_NAME))?;
    remove_file_or_directory(&template_dir.join(ANIMATION_CONFIG_INPUT_FILE_NAME))?;
    remove_file_or_directory(template_dir)?;

    Ok(())
}

/// Removes the file or directory at the given path, if it exists. Returns an error if the file or
/// directory doesn't exist, or if it is a directory and is not empty.
fn remove_file_or_directory(path: &Path) -> Result<(), ZagreusError> {
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
