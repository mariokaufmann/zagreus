use std::path::{Path, PathBuf};

use anyhow::Context;

#[cfg(test)]
pub mod temp;

pub const ASSETS_SUBFOLDER_NAME: &str = "assets";
const ORGANIZATION_SUBFOLDER_NAME: &str = "zagreus";
const LOGS_SUBFOLDER_NAME: &str = "logs";

pub fn get_log_folder_path(application_name: &str) -> anyhow::Result<PathBuf> {
    let mut folder = get_application_folder(application_name)?;
    folder.push(LOGS_SUBFOLDER_NAME);
    create_if_necessary(&folder)?;
    Ok(folder)
}

pub fn get_application_folder(application_name: &str) -> anyhow::Result<PathBuf> {
    let mut folder = get_profile_folder()?;
    let organization_subfolder = ".".to_owned() + ORGANIZATION_SUBFOLDER_NAME;
    folder.push(organization_subfolder);
    folder.push(application_name);

    create_if_necessary(&folder)?;
    Ok(folder)
}

pub fn get_assets_folder(data_folder_path: &Path) -> anyhow::Result<PathBuf> {
    let folder = data_folder_path.join(ASSETS_SUBFOLDER_NAME);
    create_if_necessary(&folder)?;
    Ok(folder)
}

fn create_if_necessary(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .with_context(|| format!("Could not prepare folder {path:?}"))?;
        Ok(())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn get_profile_folder() -> anyhow::Result<PathBuf> {
    const PROFILE_FOLDER_VAR: &str = "userprofile";
    let path = std::env::var(PROFILE_FOLDER_VAR)
        .context("User profile environment variable was not set.")?;
    Ok(PathBuf::from(path))
}

#[cfg(not(target_os = "windows"))]
fn get_profile_folder() -> anyhow::Result<PathBuf> {
    const USER_FOLDER_VAR: &str = "HOME";
    let path = std::env::var(USER_FOLDER_VAR).context("Home environment variable was not set.")?;
    Ok(PathBuf::from(path))
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
