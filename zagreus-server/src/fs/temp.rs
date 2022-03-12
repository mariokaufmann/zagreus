use std::path::PathBuf;

use anyhow::anyhow;

const TEMP_FOLDER_NAME: &str = "zagreus_temp";

pub fn prepare_temp_folder() -> anyhow::Result<PathBuf> {
    let mut temp_dir = std::env::temp_dir();
    let folder_suffix = rand::random::<u16>();
    let folder_name = format!("{}{}", TEMP_FOLDER_NAME, folder_suffix);
    temp_dir.push(folder_name);
    if temp_dir.exists() {
        return Err(anyhow!("The path already exists."));
    }
    std::fs::create_dir(&temp_dir)?;
    Ok(temp_dir)
}

pub fn delete_temp_folder(path: &PathBuf) -> anyhow::Result<()> {
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
