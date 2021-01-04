use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::data::validation::{ConfigValidate, ValidationData};
use crate::error::ZagreusError;
use crate::new::TemplateDefault;
use crate::ZAGREUS_GENERATOR_VERSION;
use std::fs::File;

pub mod animation;
pub mod element;
pub mod validation;

const DEFAULT_DEV_SERVER_ADDRESS: &str = "localhost";
const DEFAULT_DEV_SERVER_PORT: u16 = 58179;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    pub name: String,
    pub dev_server: DevServerConfig,
}

impl TemplateDefault for TemplateConfig {
    fn template_default(template_name: &str) -> Self {
        TemplateConfig {
            name: String::from(template_name),
            dev_server: DevServerConfig {
                address: String::from(DEFAULT_DEV_SERVER_ADDRESS),
                port: DEFAULT_DEV_SERVER_PORT,
            },
        }
    }
}

impl ConfigValidate for TemplateConfig {
    fn validate(&self, _: &ValidationData) -> Result<(), ZagreusError> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DevServerConfig {
    pub address: String,
    pub port: u16,
}

/// Contains meta information about the Zagreus generator and the build process.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetaInfo {
    pub zagreus_generator_version: String,
}

impl MetaInfo {
    pub fn new() -> Self {
        MetaInfo {
            zagreus_generator_version: String::from(ZAGREUS_GENERATOR_VERSION),
        }
    }
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

/// Creates the meta data JSON file in the build directory. Returns an error of the file can't be
/// created or written to, or if there is an error during serialization
///
/// # Arguments
/// * `build_folder`: The path to the template's build folder.
/// * `file_name`: The name of the meta data file.
pub fn create_meta_file(build_folder: &Path, file_name: &str) -> Result<(), ZagreusError> {
    let meta_file = File::create(build_folder.join(file_name))?;
    serde_json::to_writer(meta_file, &MetaInfo::new())?;
    Ok(())
}
