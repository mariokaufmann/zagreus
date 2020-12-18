use std::path::Path;

use crate::data::animation::config::{AnimationConfig, AnimationSequence};
use crate::data::config::TemplateConfig;
use crate::data::text::config::{TextConfig, TextElementConfig};
use crate::error::ZagreusError;
use crate::fs::get_template_folder;
use serde::de::DeserializeOwned;

pub mod event;
pub mod registry;

const ANIMATIONS_FILE_NAME: &str = "animations.json";
const TEXTS_FILE_NAME: &str = "texts.json";
const TEMPLATE_CONFIG_FILE_NAME: &str = "template.json";

pub struct Template {
    pub name: String,
    pub animations: Vec<AnimationSequence>,
    pub text_elements: Vec<TextElementConfig>,
    pub template: TemplateConfig,
}

impl Template {
    pub fn load(data_folder: &Path, template_name: &str) -> Result<Template, ZagreusError> {
        let template_folder = get_template_folder(data_folder, template_name)?;
        let animation_config = Self::load_config::<AnimationConfig>(&template_folder, ANIMATIONS_FILE_NAME)?;
        let texts_config = Self::load_config::<TextConfig>(&template_folder, TEXTS_FILE_NAME)?;
        let template_config = Self::load_config(&template_folder, TEMPLATE_CONFIG_FILE_NAME)?;

        let template = Template {
            name: String::from(template_name),
            animations: animation_config.sequences,
            text_elements: texts_config.elements,
            template: template_config,
        };
        Ok(template)
    }

    fn load_config<T>(template_folder: &Path, input_file_name: &str) -> Result<T, ZagreusError> where T: DeserializeOwned {
        let file_path = template_folder.join(input_file_name);
        let file = std::fs::File::open(file_path)?;
        let config: T = serde_json::from_reader(file)?;
        Ok(config)
    }
}