use std::path::Path;

use serde::de::DeserializeOwned;

use crate::data::animation::config::{AnimationConfig, AnimationSequence};
use crate::data::config::{TemplateConfig, TemplateElement, TemplateElements};
use crate::error::ZagreusError;
use crate::fs::get_template_folder;

pub mod event;
pub mod registry;

const ANIMATIONS_FILE_NAME: &str = "animations.json";
const ELEMENTS_FILE_NAME: &str = "elements.json";
const TEMPLATE_CONFIG_FILE_NAME: &str = "template.json";

pub struct Template {
    pub name: String,
    pub animations: Vec<AnimationSequence>,
    pub elements: Vec<TemplateElement>,
    pub template: TemplateConfig,
}

impl Template {
    pub fn load(data_folder: &Path, template_name: &str) -> Result<Template, ZagreusError> {
        let template_folder = get_template_folder(data_folder, template_name)?;
        let animation_config =
            Self::load_config::<AnimationConfig>(&template_folder, ANIMATIONS_FILE_NAME)?;
        let element_configs =
            Self::load_config::<TemplateElements>(&template_folder, ELEMENTS_FILE_NAME)?;
        let template_config = Self::load_config(&template_folder, TEMPLATE_CONFIG_FILE_NAME)?;

        let template = Template {
            name: String::from(template_name),
            animations: animation_config.sequences,
            elements: element_configs.elements,
            template: template_config,
        };
        Ok(template)
    }

    fn load_config<T>(template_folder: &Path, input_file_name: &str) -> Result<T, ZagreusError>
    where
        T: DeserializeOwned,
    {
        let file_path = template_folder.join(input_file_name);
        let file = std::fs::File::open(file_path)?;
        let config: T = serde_json::from_reader(file)?;
        Ok(config)
    }
}
