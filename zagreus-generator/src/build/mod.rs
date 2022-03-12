use std::path::{Path, PathBuf};

use crate::data::animation::config::AnimationConfig;
use crate::data::element::{merge_elements_with_config, ElementsConfig, TemplateElements};
use crate::data::validation::ValidationData;
use crate::data::TemplateConfig;
use crate::error::error_with_message;

mod asset;
mod html;
mod svg;
mod transform;
mod zip;

pub const BUILD_FOLDER_NAME: &str = "build";
pub const ASSETS_FOLDER_NAME: &str = "assets";
pub const ANIMATION_CONFIG_INPUT_FILE_NAME: &str = "animations.yaml";
pub const ELEMENT_CONFIG_INPUT_FILE_NAME: &str = "elements.yaml";
pub const INPUT_SVG_FILE_NAME: &str = "template.svg";

const ZIPPED_TEMPLATE_FILE_NAME: &str = "template.zip";
const PROCESSED_SVG_FILE_NAME: &str = "template_processed.svg";
const RAW_HTML_FILE_NAME: &str = "index_raw.html";
const HTML_FILE_NAME: &str = "index.html";

const ELEMENTS_OUTPUT_FILE_NAME: &str = "elements.json";
const TEMPLATE_CONFIG_OUTPUT_FILE_NAME: &str = "template.json";
const ANIMATION_CONFIG_OUTPUT_FILE_NAME: &str = "animations.json";
const META_INFO_OUTPUT_FILE_NAME: &str = "meta.json";

pub fn build_template(build_folder: &Path, template_config: &TemplateConfig) -> anyhow::Result<()> {
    if !build_folder.exists() {
        if let Err(err) = std::fs::create_dir(build_folder) {
            return error_with_message("Could not create build folder", err);
        }
    }

    // Create the meta data file.
    crate::data::create_meta_file(build_folder, META_INFO_OUTPUT_FILE_NAME)?;

    let input_template_file_path = Path::new(INPUT_SVG_FILE_NAME);
    let processed_template_file_path = build_folder.join(PROCESSED_SVG_FILE_NAME);

    let template_elements =
        svg::process_svg(input_template_file_path, &processed_template_file_path)?;

    let collected_stylesheets = asset::collect_stylesheets(Path::new("./")).unwrap();

    let raw_html_path = build_folder.join(RAW_HTML_FILE_NAME);
    html::write_raw_html(
        &processed_template_file_path,
        &raw_html_path,
        template_config,
        collected_stylesheets,
    );

    let processed_html_path = build_folder.join(HTML_FILE_NAME);
    html::process_raw_html(&raw_html_path, &processed_html_path);

    let validation_data = ValidationData {
        template_elements: &template_elements,
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

    // process elements
    let elements_output_path = build_folder.join(ELEMENTS_OUTPUT_FILE_NAME);
    let cloned_elements = template_elements.clone();
    if let Err(err) = crate::data::map_and_convert_config::<ElementsConfig, TemplateElements, _>(
        Path::new(ELEMENT_CONFIG_INPUT_FILE_NAME),
        &elements_output_path,
        &validation_data,
        move |configs| {
            let mut elements = cloned_elements;
            merge_elements_with_config(&mut elements, configs);
            elements
        },
    ) {
        return error_with_message("Could not convert elements", err);
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

    let build_files: Vec<PathBuf> = vec![
        HTML_FILE_NAME,
        TEMPLATE_CONFIG_OUTPUT_FILE_NAME,
        ANIMATION_CONFIG_OUTPUT_FILE_NAME,
        ELEMENTS_OUTPUT_FILE_NAME,
    ]
    .iter()
    .map(|file_name| build_folder.join(file_name))
    .collect();
    let assets_folder = PathBuf::from(ASSETS_FOLDER_NAME);
    let packed_file_path = get_zipped_template_file_path(build_folder);
    zip::pack_template(&packed_file_path, &build_files, &assets_folder).unwrap();

    Ok(())
}

pub fn get_zipped_template_file_path(build_folder: &Path) -> PathBuf {
    build_folder.join(ZIPPED_TEMPLATE_FILE_NAME)
}

#[cfg(test)]
mod tests {
    pub(crate) fn assert_equal_ignoring_newlines(left: String, right: String) {
        let left = remove_newlines(left);
        let right = remove_newlines(right);
        assert_eq!(left, right);
    }

    fn remove_newlines(text: String) -> String {
        text.chars().filter(|x| *x != '\n' && *x != '\r').collect()
    }
}
