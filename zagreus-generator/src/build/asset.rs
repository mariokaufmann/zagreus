use std::collections::HashSet;
use std::path::Path;

use crate::build::ASSETS_FOLDER_NAME;
use crate::error::ZagreusError;

pub fn collect_stylesheets<P: AsRef<Path>>(
    base_folder: P,
) -> Result<HashSet<String>, ZagreusError> {
    let assets_folder = base_folder.as_ref().join(ASSETS_FOLDER_NAME);
    let read_dir = std::fs::read_dir(assets_folder)?;
    let entries = read_dir
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| match path.extension() {
            Some(extension) => extension.eq("css"),
            None => false,
        })
        .filter_map(|path| path.file_name().map(|file_name| file_name.to_os_string()))
        .map(|name| name.into_string())
        .filter_map(Result::ok)
        .collect();
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::fs::temp::TempFolder;

    use super::*;

    #[test]
    fn collect_stylesheets_valid() {
        let temp_folder = TempFolder::new().unwrap();
        let asset_folder = temp_folder.join(ASSETS_FOLDER_NAME);
        std::fs::create_dir(&asset_folder).unwrap();
        let stylesheet1_path = asset_folder.join("main.css");
        std::fs::File::create(stylesheet1_path).unwrap();
        let stylesheet2_path = asset_folder.join("animations.css");
        std::fs::File::create(stylesheet2_path).unwrap();

        let stylesheets = collect_stylesheets(temp_folder).unwrap();

        assert_eq!(stylesheets.len(), 2);
        assert!(stylesheets.contains("main.css"));
        assert!(stylesheets.contains("animations.css"));
    }
}
