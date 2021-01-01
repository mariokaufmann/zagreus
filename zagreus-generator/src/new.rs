use crate::build::{
    ANIMATION_CONFIG_INPUT_FILE_NAME, ASSETS_FOLDER_NAME, ELEMENT_CONFIG_INPUT_FILE_NAME,
};
use crate::data::animation::config::AnimationConfig;
use crate::data::element::ElementsConfig;
use crate::data::TemplateConfig;
use crate::error::{simple_error, ZagreusError};
use crate::TEMPLATE_CONFIG_FILE_NAME;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::path::Path;

/// A trait for creating useful default instances of template-related types, similar to the
/// `core::default::Default` trait.
pub trait TemplateDefault {
    /// Returns the default value for a template-related type.
    ///
    /// # Arguments
    /// * `template_name`: The name of the template for which this instance belongs
    fn template_default(template_name: &str) -> Self;
}

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
/// message containing the complete set of offending characters. Also returns an error if the
/// template name is empty.
fn validate_template_name(template_name: &str) -> Result<(), ZagreusError> {
    if template_name.is_empty() {
        return simple_error("Template name must not be empty");
    }

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
    create_default_config::<TemplateConfig>(
        template_dir,
        TEMPLATE_CONFIG_FILE_NAME,
        template_name,
    )?;
    create_default_config::<AnimationConfig>(
        template_dir,
        ANIMATION_CONFIG_INPUT_FILE_NAME,
        template_name,
    )?;
    create_default_config::<ElementsConfig>(
        template_dir,
        ELEMENT_CONFIG_INPUT_FILE_NAME,
        template_name,
    )?;

    Ok(())
}

/// Creates a default config instance of type T and serializes it to a new file with the given name,
/// at the root of the template directory. Returns an error if the config instance cannot be
/// serialized or written to the corresponding file.
fn create_default_config<T>(
    template_dir: &Path,
    file_name: &str,
    template_name: &str,
) -> Result<(), ZagreusError>
where
    T: Serialize + TemplateDefault,
{
    let config = T::template_default(template_name);
    write_to_new_file(&template_dir.join(file_name), &config)?;
    Ok(())
}

/// Creates a new file at the given path, serializes `config` to YAML, and writes the result into
/// the newly created file. Returns an error of the file already exists or an IO error occurs.
fn write_to_new_file<T>(file_path: &Path, config: &T) -> Result<(), ZagreusError>
where
    T: Serialize,
{
    if file_path.exists() {
        // Reaching here is considered a bug: a new (i.e. empty) template directory should be
        // created first, and no file should be created more than once.
        return simple_error(&format!("File already exists: {:?}", file_path));
    }
    let file = File::create(file_path)?;
    serde_yaml::to_writer(file, config)?;
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
