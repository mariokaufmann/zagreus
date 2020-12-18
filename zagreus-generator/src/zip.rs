use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::error::ZagreusError;
use crate::get_path_in_build_folder;

const ZIPPED_TEMPLATE_FILE_NAME: &str = "template.zip";

pub fn pack_template(build_file_names: Vec<&str>, assets_folder: &PathBuf) -> Result<(), ZagreusError> {
    debug!("Packing template.");
    let assets_walkdir = WalkDir::new(assets_folder);
    let zipped_template_file = get_path_in_build_folder(ZIPPED_TEMPLATE_FILE_NAME);
    let zipped_file = std::fs::File::create(zipped_template_file)?;
    let mut zip_writer = zip::ZipWriter::new(zipped_file);

    let mut buffer = Vec::new();

    // pack build files
    debug!("Packing build files.");
    for build_file_name in build_file_names {
        let build_file = get_path_in_build_folder(build_file_name);
        debug!("Packing build file: {}.", build_file.display());
        write_zip_file(&mut zip_writer, build_file, build_file_name, &mut buffer)?;
    }

    // pack assets
    debug!("Packing assets.");
    for entry in assets_walkdir {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                match path.to_str() {
                    Some(output_file_name) => {
                        if path.is_dir() {
                            debug!("Packing asset directory {}.", output_file_name);
                            zip_writer.add_directory(output_file_name, get_file_options())?;
                        } else {
                            debug!("Packing asset {}.", path.display());
                            write_zip_file(&mut zip_writer, path, output_file_name, &mut buffer)?;
                        }
                    }
                    None => error!("Could not convert file name {} to string.", path.display()),
                }
            }
            Err(err) => warn!("Could not pack file into zip: {}.", err),
        }
    }

    zip_writer.finish()?;
    Ok(())
}

fn write_zip_file<P, S>(zip_writer: &mut ZipWriter<std::fs::File>, input_file_path: P, output_file: S, buffer: &mut Vec<u8>) -> Result<(), ZagreusError>
    where P: AsRef<Path>, S: Into<String> {
    zip_writer.start_file(output_file, get_file_options())?;
    let mut input_file = std::fs::File::open(input_file_path)?;

    input_file.read_to_end(buffer)?;
    zip_writer.write_all(buffer)?;
    buffer.clear();

    Ok(())
}

fn get_file_options() -> FileOptions {
    FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755)
}