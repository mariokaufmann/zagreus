use std::path::PathBuf;

use crate::error::ZagreusError;

const TEMP_FOLDER_NAME: &str = "zagreus_temp";

pub fn prepare_temp_folder() -> Result<PathBuf, ZagreusError> {
    let mut temp_dir = std::env::temp_dir();
    let folder_suffix = rand::random::<u16>();
    let folder_name = format!("{}{}", TEMP_FOLDER_NAME, folder_suffix);
    temp_dir.push(folder_name);
    if temp_dir.exists() {
        return Err(ZagreusError::new("The path already exists."));
    }
    std::fs::create_dir(&temp_dir)?;
    Ok(temp_dir)
}

pub fn delete_temp_folder(path: &PathBuf) -> Result<(), ZagreusError> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_folder() {
        let folder = prepare_temp_folder().unwrap();
        assert!(folder.exists());
        delete_temp_folder(&folder).unwrap();
        assert!(!folder.exists());
    }
}