use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::data::validation::{ConfigValidate, ValidationData};
use crate::error::ZagreusError;
use crate::get_path_in_build_folder;

pub mod animation;
pub mod text;
pub mod validation;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    pub name: String,
    pub on_load: OnLoadConfig,
    pub dev_server: DevServerConfig,
}

impl ConfigValidate for TemplateConfig {
    fn validate(&self, _: &ValidationData) -> Result<(), ZagreusError> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnLoadConfig {
    pub animation_sequences: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataElements {
    elements: Vec<String>,
}

impl DataElements {
    pub fn new(elements: Vec<String>) -> DataElements {
        DataElements {
            elements
        }
    }

    pub fn has_data_element(&self, element_name: &str) -> bool {
        self.elements.iter().any(|element| (*element).eq(element_name))
    }
}

pub fn convert_config<T>(config_file_path: &Path, output_file_name: &str, validation_data: &ValidationData) -> Result<(), ZagreusError> where T: DeserializeOwned + Serialize + ConfigValidate {
    let config: T = load_config(config_file_path)?;
    config.validate(validation_data)?;
    let converted_file_path = get_path_in_build_folder(output_file_name);
    let output_file = std::fs::File::create(&converted_file_path)?;
    serde_json::to_writer_pretty(output_file, &config)?;
    Ok(())
}

pub fn load_config<T>(config_file_path: &Path) -> Result<T, ZagreusError> where T: DeserializeOwned + Serialize + ConfigValidate {
    let input_file = std::fs::File::open(&config_file_path)?;
    let config: T = serde_yaml::from_reader(input_file)?;
    Ok(config)
}

