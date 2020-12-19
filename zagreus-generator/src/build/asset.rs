use crate::error::ZagreusError;

pub const ASSETS_FOLDER_NAME: &str = "assets";

pub fn collect_stylesheets() -> Result<Vec<String>, ZagreusError> {
    let read_dir = std::fs::read_dir(ASSETS_FOLDER_NAME)?;
    let entries = read_dir.into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            match path.extension() {
                Some(extension) => extension.eq("css"),
                None => false,
            }
        })
        .filter_map(|path| path.file_name().map(|file_name| file_name.to_os_string()))
        .map(|name| name.into_string())
        .filter_map(Result::ok)
        .collect();
    Ok(entries)
}