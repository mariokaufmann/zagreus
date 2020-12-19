use std::path::{Path, PathBuf};

use crate::error::ZagreusError;

#[cfg(test)]
pub mod temp;
pub mod zip;

pub const TEMPLATES_DATA_SUBFOLDER_NAME: &str = "templates";
const ORGANIZATION_SUBFOLDER_NAME: &str = "zagreus";
const LOGS_SUBFOLDER_NAME: &str = "logs";

pub fn get_log_folder_path(application_name: &str) -> Result<PathBuf, ZagreusError> {
    let mut folder = get_application_folder(application_name)?;
    folder.push(LOGS_SUBFOLDER_NAME);
    create_if_necessary(&folder)?;
    Ok(folder)
}

pub fn get_application_folder(application_name: &str) -> Result<PathBuf, ZagreusError> {
    let mut folder = get_profile_folder()?;
    let organization_subfolder = ".".to_owned() + ORGANIZATION_SUBFOLDER_NAME;
    folder.push(organization_subfolder);
    folder.push(application_name);

    create_if_necessary(&folder)?;
    Ok(folder)
}

pub fn get_templates_data_folder(data_folder_path: &Path) -> Result<PathBuf, ZagreusError> {
    let folder = data_folder_path.join(TEMPLATES_DATA_SUBFOLDER_NAME);
    create_if_necessary(&folder)?;
    Ok(folder)
}

pub fn get_template_folder(data_folder_path: &Path, template_name: &str) -> Result<PathBuf, ZagreusError> {
    let mut folder = get_templates_data_folder(data_folder_path)?;
    folder.push(template_name);
    create_if_necessary(&folder)?;
    Ok(folder)
}

fn create_if_necessary(path: &Path) -> Result<(), ZagreusError> {
    if !path.exists() {
        match std::fs::create_dir_all(path) {
            Ok(()) => Ok(()),
            Err(err) => {
                let message = format!("Could not prepare folder{:?}: {}.", path, err);
                Err(ZagreusError::from(message))
            }
        }
    } else {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn get_profile_folder() -> Result<PathBuf, ZagreusError> {
    const PROFILE_FOLDER_VAR: &str = "userprofile";
    match std::env::var(PROFILE_FOLDER_VAR) {
        Ok(var) => Ok(PathBuf::from(var)),
        Err(err) => {
            Err(ZagreusError::from("User profile environment variable was not set: ".to_owned() + err.to_string().as_str()))
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_profile_folder() -> Result<PathBuf, ZagreusError> {
    const USER_FOLDER_VAR: &str = "HOME";
    match std::env::var(USER_FOLDER_VAR) {
        Ok(var) => Ok(PathBuf::from(var)),
        Err(err) => {
            Err(ZagreusError::from(format!("Home environment variable was not set: {}", err)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_profile_folder() {
        let path = get_profile_folder().unwrap();
        assert!(path.exists());
    }
}