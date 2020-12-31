use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::data::validation::{ConfigValidate, ValidationData};
use crate::error::ZagreusError;

pub mod animation;
pub mod element;
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

/// Reads the config of type `T` from the input file, validates it and outputs
/// it to the output file.
pub fn convert_config<T>(
    input_file_path: &Path,
    output_file_path: &Path,
    validation_data: &ValidationData,
) -> Result<(), ZagreusError>
where
    T: DeserializeOwned + Serialize + ConfigValidate,
{
    let config: T = load_config(input_file_path)?;
    config.validate(validation_data)?;
    let output_file = std::fs::File::create(&output_file_path)?;
    serde_json::to_writer_pretty(output_file, &config)?;
    Ok(())
}

/// Reads the config of type `I` from the input file, validates it,
/// maps it to a config of type `O` and outputs it to the output file.
pub fn map_and_convert_config<I, O, F>(
    input_file_path: &Path,
    output_file_path: &Path,
    validation_data: &ValidationData,
    mapping_fun: F,
) -> Result<(), ZagreusError>
where
    I: DeserializeOwned + ConfigValidate,
    O: Serialize,
    F: FnOnce(I) -> O,
{
    let config: I = load_config(input_file_path)?;
    config.validate(validation_data)?;
    let mapped_config = mapping_fun(config);
    let output_file = std::fs::File::create(&output_file_path)?;
    serde_json::to_writer_pretty(output_file, &mapped_config)?;
    Ok(())
}

pub fn load_config<T>(config_file_path: &Path) -> Result<T, ZagreusError>
where
    T: DeserializeOwned + ConfigValidate,
{
    let input_file = std::fs::File::open(&config_file_path)?;
    let config: T = serde_yaml::from_reader(input_file)?;
    Ok(config)
}
