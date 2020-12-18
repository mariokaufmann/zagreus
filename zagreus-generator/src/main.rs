#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriterEvent;

use crate::data::{DataElements, TemplateConfig};
use crate::data::animation::config::AnimationConfig;
use crate::data::text::config::TextConfig;
use crate::data::validation::ValidationData;

mod asset;
mod data;
mod error;
mod html;
mod logger;
mod transform;
mod upload;
mod zip;

const BUILD_FOLDER_NAME: &str = "build";

fn main() {
    logger::init_logger();
    info!("Start processing.");

    info!("Loading template config.");
    let template_config = crate::data::load_config::<TemplateConfig>(Path::new("zagreus-template.yaml")).unwrap();

    let reader = transform::create_xml_reader(&PathBuf::from("template.svg"));

    let build_folder_path = Path::new(BUILD_FOLDER_NAME);
    if !build_folder_path.exists() {
        std::fs::create_dir(build_folder_path).unwrap();
    }

    let processed_template_file_path = get_path_in_build_folder("template_processed.svg");
    let mut processed_template_writer = transform::create_xml_writer(&processed_template_file_path);

    let mut found_elements = Vec::new();
    let data_file_path = get_path_in_build_folder("data.json");

    for e in reader {
        match e {
            Ok(evt) => {
                if let ReaderEvent::StartElement { name: _, attributes, namespace: _ } = &evt {
                    for attribute in attributes {
                        if attribute.name.local_name.eq("id") {
                            found_elements.push(attribute.value.clone());
                        }
                    }
                }

                match transform_event(&evt) {
                    Some(transformed_event) => {
                        if let Err(err) = processed_template_writer.write(transformed_event) {
                            error!("Could not write event to output SVG file: {}.", err);
                        }
                    }
                    None => warn!("Could not transform event, it is skipped."),
                }
            }
            Err(err) => {
                error!("Could not read XML event: {}.", err);
                break;
            }
        }
    }

    let data_elements = DataElements::new(found_elements);
    match serde_json::to_string_pretty(&data_elements) {
        Ok(serialized_data) => {
            if let Err(err) = std::fs::write(data_file_path, serialized_data) {
                error!("Could not write data.json file: {}.", err);
            }
        }
        Err(err) => error!("Could not serialize data elements: {}.", err),
    }

    let collected_stylesheets = crate::asset::collect_stylesheets().unwrap();

    let raw_html_path = get_path_in_build_folder("index_raw.html");
    html::write_raw_html(&processed_template_file_path, &raw_html_path, &template_config.name, &collected_stylesheets);

    let processed_html_path = get_path_in_build_folder("index.html");
    html::process_raw_html(&raw_html_path, &processed_html_path);

    let validation_data = ValidationData {
        data_elements: &data_elements,
    };

    // process template config
    if let Err(err) = data::convert_config::<TemplateConfig>(Path::new("zagreus-template.yaml"), "template.json", &validation_data) {
        error!("Could not convert template config: {}.", err);
        return;
    }

    // process animations
    if let Err(err) = data::convert_config::<AnimationConfig>(Path::new("animations.yaml"), "animations.json", &validation_data) {
        error!("Could not convert animations: {}.", err);
        return;
    }

    // process text
    if let Err(err) = data::convert_config::<TextConfig>(Path::new("texts.yaml"), "texts.json", &validation_data) {
        error!("Could not convert texts: {}.", err);
        return;
    }

    let build_files = vec!["index.html", "data.json", "template.json", "animations.json", "texts.json"];
    let assets_folder = PathBuf::from(crate::asset::ASSETS_FOLDER_NAME);
    zip::pack_template(build_files, &assets_folder).unwrap();

    match upload::TemplateUploader::new(&format!("{}:{}", &template_config.dev_server.address, &template_config.dev_server.port), &template_config.name, "template.zip") {
        Ok(template_uploader) => {
            if let Err(err) = template_uploader.upload_template() {
                error!("Could not upload template: {}.", err);
            }
        }
        Err(err) => error!("Could not construct template uploader: {}.", err),
    }

    info!("Finished processing.");
}

fn transform_event(event: &ReaderEvent) -> Option<WriterEvent> {
    match event {
        ReaderEvent::StartDocument { .. } => None,
        ReaderEvent::StartElement { name, attributes, namespace } => {
            let attributes = attributes.iter().map(|attribute| attribute.borrow()).collect();
            Option::Some(WriterEvent::StartElement { name: name.borrow(), attributes, namespace: Cow::Borrowed(namespace) })
        }
        other => other.as_writer_event()
    }
}

fn get_path_in_build_folder(file_name: &str) -> PathBuf {
    [BUILD_FOLDER_NAME, file_name].iter().collect()
}