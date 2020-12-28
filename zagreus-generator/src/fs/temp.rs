use std::path::{Path, PathBuf};

use crate::error::ZagreusError;

const TEMP_FOLDER_NAME: &str = "zagreus_temp";

pub struct TempFolder {
    path: PathBuf,
}

impl TempFolder {
    pub fn new() -> Result<TempFolder, ZagreusError> {
        let path = Self::prepare_temp_folder()?;
        Ok(TempFolder { path })
    }

    fn prepare_temp_folder() -> Result<PathBuf, ZagreusError> {
        let mut temp_dir = std::env::temp_dir();
        let folder_suffix = rand::random::<u16>();
        let folder_name = format!("{}{}", TEMP_FOLDER_NAME, folder_suffix);
        temp_dir.push(folder_name);
        if temp_dir.exists() {
            return Err(ZagreusError::from("The path already exists.".to_owned()));
        }
        std::fs::create_dir(&temp_dir)?;
        Ok(temp_dir)
    }

    fn delete_temp_folder(&self) -> Result<(), ZagreusError> {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path)?;
        }
        Ok(())
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.path.join(path)
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        self.delete_temp_folder().expect(&format!(
            "Could not delete temp folder {}.",
            &self.path.display()
        ));
    }
}

impl AsRef<Path> for TempFolder {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_folder() {
        let folder = TempFolder::new().unwrap();
        assert!(folder.path.exists());
        let path = folder.path.clone();
        drop(folder);
        assert!(!path.exists());
    }
}
