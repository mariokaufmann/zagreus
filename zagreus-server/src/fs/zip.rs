use std::io::Read;
use std::path::PathBuf;

use crate::error::ZagreusError;
use crate::fs::get_template_folder;

pub fn unpack_template_files(template_name: &str, file_buffer: Vec<u8>, data_folder: &PathBuf) -> Result<(), ZagreusError> {
    debug!("Starting to unpack template: {}", template_name);
    let reader = std::io::Cursor::new(file_buffer);

    let mut zip = zip::ZipArchive::new(reader)?;
    let template_folder = get_template_folder(data_folder, template_name)?;

    debug!("Unzipping to {:?}.", &template_folder);

    for file_index in 0..zip.len() {
        let mut file = zip.by_index(file_index)?;
        let mut output_file_path = template_folder.clone();
        let file_path = PathBuf::from(file.name());
        output_file_path.push(file_path);

        if file.name().ends_with("/") {
            debug!("Creating directory: {:?}.", output_file_path);
            if !output_file_path.exists() {
                std::fs::create_dir_all(output_file_path)?;
            }
        } else {
            debug!("Unzipping file to {:?}.", &output_file_path);
            let mut file_contents = Vec::new();
            file.read_to_end(&mut file_contents)?;
            std::fs::write(output_file_path, file_contents)?;
        }
    }

    Ok(())
}