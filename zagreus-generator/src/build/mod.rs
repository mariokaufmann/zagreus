use std::path::{Path, PathBuf};

use crate::data::animation::config::AnimationConfig;
use crate::data::text::config::TextConfig;
use crate::data::validation::ValidationData;
use crate::data::TemplateConfig;
use crate::error::{error_with_message, ZagreusError};

mod asset;
mod html;
mod svg;
mod transform;
mod zip;

pub const BUILD_FOLDER_NAME: &str = "build";
pub const ASSETS_FOLDER_NAME: &str = "assets";
pub const ANIMATION_CONFIG_INPUT_FILE_NAME: &str = "animations.yaml";

const ZIPPED_TEMPLATE_FILE_NAME: &str = "template.zip";
const INPUT_SVG_FILE_NAME: &str = "template.svg";
const PROCESSED_SVG_FILE_NAME: &str = "template_processed.svg";
const RAW_HTML_FILE_NAME: &str = "index_raw.html";
const HTML_FILE_NAME: &str = "index.html";

const TEXT_CONFIG_INPUT_FILE_NAME: &str = "texts.yaml";

const DATA_OUTPUT_FILE_NAME: &str = "data.json";
const TEMPLATE_CONFIG_OUTPUT_FILE_NAME: &str = "template.json";
const ANIMATION_CONFIG_OUTPUT_FILE_NAME: &str = "animations.json";
const TEXT_CONFIG_OUTPUT_FILE_NAME: &str = "texts.json";

pub fn build_template(
    build_folder: &Path,
    template_config: &TemplateConfig,
) -> Result<(), ZagreusError> {
    info!("Building template {}...", &template_config.name);

    if !build_folder.exists() {
        if let Err(err) = std::fs::create_dir(build_folder) {
            return error_with_message("Could not create build folder", err);
        }
    }

    let input_template_file_path = Path::new(INPUT_SVG_FILE_NAME);
    let processed_template_file_path = build_folder.join(PROCESSED_SVG_FILE_NAME);

    let data_elements = svg::process_svg(&input_template_file_path, &processed_template_file_path)?;

    let data_file_path = build_folder.join(DATA_OUTPUT_FILE_NAME);
    match serde_json::to_string_pretty(&data_elements) {
        Ok(serialized_data) => {
            if let Err(err) = std::fs::write(data_file_path, serialized_data) {
                error!("Could not write data.json file: {}.", err);
            }
        }
        Err(err) => error!("Could not serialize data elements: {}.", err),
    }

    let collected_stylesheets = asset::collect_stylesheets(Path::new("./")).unwrap();

    let raw_html_path = build_folder.join(RAW_HTML_FILE_NAME);
    html::write_raw_html(
        &processed_template_file_path,
        &raw_html_path,
        &template_config.name,
        collected_stylesheets,
    );

    let processed_html_path = build_folder.join(HTML_FILE_NAME);
    html::process_raw_html(&raw_html_path, &processed_html_path);

    let validation_data = ValidationData {
        data_elements: &data_elements,
    };

    // process template config
    let template_config_output_path = build_folder.join(TEMPLATE_CONFIG_OUTPUT_FILE_NAME);
    if let Err(err) = crate::data::convert_config::<TemplateConfig>(
        Path::new(crate::TEMPLATE_CONFIG_FILE_NAME),
        &template_config_output_path,
        &validation_data,
    ) {
        error!(": {}.", err);
        return error_with_message("Could not convert template config", err);
    }

    // process animations
    let animation_config_output_path = build_folder.join(ANIMATION_CONFIG_OUTPUT_FILE_NAME);
    if let Err(err) = crate::data::convert_config::<AnimationConfig>(
        Path::new(ANIMATION_CONFIG_INPUT_FILE_NAME),
        &animation_config_output_path,
        &validation_data,
    ) {
        return error_with_message("Could not convert animations", err);
    }

    // process text
    let text_config_output_path = build_folder.join(TEXT_CONFIG_OUTPUT_FILE_NAME);
    if let Err(err) = crate::data::convert_config::<TextConfig>(
        Path::new(TEXT_CONFIG_INPUT_FILE_NAME),
        &text_config_output_path,
        &validation_data,
    ) {
        return error_with_message("Could not convert texts", err);
    }

    let build_files: Vec<PathBuf> = vec![
        HTML_FILE_NAME,
        DATA_OUTPUT_FILE_NAME,
        TEMPLATE_CONFIG_OUTPUT_FILE_NAME,
        ANIMATION_CONFIG_OUTPUT_FILE_NAME,
        TEXT_CONFIG_OUTPUT_FILE_NAME,
    ]
    .iter()
    .map(|file_name| build_folder.join(file_name))
    .collect();
    let assets_folder = PathBuf::from(ASSETS_FOLDER_NAME);
    let packed_file_path = get_zipped_template_file_path(build_folder);
    zip::pack_template(&packed_file_path, &build_files, &assets_folder).unwrap();

    info!("Finished building template {}.", &template_config.name);

    Ok(())
}

pub fn get_zipped_template_file_path(build_folder: &Path) -> PathBuf {
    build_folder.join(ZIPPED_TEMPLATE_FILE_NAME)
}
