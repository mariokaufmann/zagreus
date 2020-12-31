use crate::data::validation::{get_duplicate_elements, ConfigValidate, ValidationData};
use crate::error::{error_with_message, simple_error, ZagreusError};

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateElements {
    elements: Vec<TemplateElement>,
}

impl TemplateElements {
    pub fn from_ids(mut element_ids: Vec<String>) -> TemplateElements {
        let elements = element_ids
            .drain(..)
            .map(|id| TemplateElement { id, config: None })
            .collect();
        TemplateElements { elements }
    }

    pub fn has_template_element(&self, element_id: &str) -> bool {
        self.elements
            .iter()
            .any(|element| (*element.id).eq(element_id))
    }

    /// Merges these template elements with their corresponding
    /// element configs (if available)
    pub fn merge_with_configs(&mut self, mut element_config: Vec<ElementConfig>) {
        for element in &mut self.elements {
            let config_index = element_config
                .iter()
                .position(|config| config.id.eq(&element.id));
            if let Some(config_index) = config_index {
                let config = element_config.remove(config_index);
                element.config = Some(config);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateElement {
    id: String,
    config: Option<ElementConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ElementsConfig {
    elements: Vec<ElementConfig>,
}

pub fn merge_elements_with_config(elements: &mut TemplateElements, config: ElementsConfig) {
    elements.merge_with_configs(config.elements);
}

impl ConfigValidate for ElementsConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        for element_config in &self.elements {
            if !validation_data
                .template_elements
                .has_template_element(&element_config.id)
            {
                return Err(ZagreusError::from(format!(
                    "Element config contains unknown element {}.",
                    &element_config.id
                )));
            }

            // check alignments
            if let Err(err) = element_config.align.validate(validation_data) {
                return error_with_message(
                    &format!("Element {} has invalid alignment", &element_config.id),
                    err,
                );
            }
        }

        // check for duplicate elements
        let duplicate_elements = get_duplicate_elements(&self.elements, |element| &element.id);
        for duplicate_element in &duplicate_elements {
            error!(
                "Element {} is configured more than once.",
                duplicate_element
            )
        }
        if !duplicate_elements.is_empty() {
            return simple_error("At least one element was configured more than once.");
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ElementConfig {
    id: String,
    align: AlignmentConfig,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AlignmentConfig {
    #[serde(default)]
    horizontal: HorizontalAlignment,
    #[serde(default)]
    vertical: VerticalAlignment,
    #[serde(default)]
    with: String,
}

impl ConfigValidate for AlignmentConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        if self.horizontal == HorizontalAlignment::Center && self.with.is_empty() {
            return Err(ZagreusError::from(
                "Element is horizontally center-aligned but no alignWith is configured."
                    .to_string(),
            ));
        }

        if self.vertical == VerticalAlignment::Center && self.with.is_empty() {
            return Err(ZagreusError::from(
                "Element is vertically center-aligned but no alignWith is configured.".to_string(),
            ));
        }

        if !self.with.is_empty()
            && !validation_data
                .template_elements
                .has_template_element(&self.with)
        {
            return Err(ZagreusError::from(format!(
                "Element is configured to be aligned with unknown element {}.",
                &self.with
            )));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Left
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlignment {
    Center,
    Top,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}

#[cfg(test)]
mod tests {
    use crate::data::validation::ValidationData;

    use super::*;

    impl AlignmentConfig {
        fn center(align_with: &str) -> AlignmentConfig {
            AlignmentConfig {
                horizontal: HorizontalAlignment::Center,
                vertical: VerticalAlignment::Center,
                with: align_with.to_owned(),
            }
        }
    }

    #[test]
    fn validate_element_config_valid() {
        let data_elements =
            TemplateElements::from_ids(vec![String::from("id1"), String::from("id2")]);
        let validation_data = ValidationData {
            template_elements: &data_elements,
        };
        let element_config = ElementsConfig {
            elements: vec![
                ElementConfig {
                    id: String::from("id1"),
                    align: AlignmentConfig::default(),
                },
                ElementConfig {
                    id: String::from("id2"),
                    align: AlignmentConfig::center("id1"),
                },
            ],
        };

        let result = element_config.validate(&validation_data);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_element_config_inexistent_element() {
        let data_elements = TemplateElements::from_ids(vec![String::from("id2")]);
        let validation_data = ValidationData {
            template_elements: &data_elements,
        };
        let element_config = ElementsConfig {
            elements: vec![ElementConfig {
                id: String::from("id1"),
                align: AlignmentConfig::default(),
            }],
        };

        let result = element_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_element_config_center_no_align_with() {
        let data_elements = TemplateElements::from_ids(vec![String::from("id1")]);
        let validation_data = ValidationData {
            template_elements: &data_elements,
        };
        let element_config = ElementsConfig {
            elements: vec![ElementConfig {
                id: String::from("id1"),
                align: AlignmentConfig::center(""),
            }],
        };

        let result = element_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_element_config_center_invalid_align_with() {
        let data_elements = TemplateElements::from_ids(vec![String::from("id1")]);
        let validation_data = ValidationData {
            template_elements: &data_elements,
        };
        let element_config = ElementsConfig {
            elements: vec![ElementConfig {
                id: String::from("id1"),
                align: AlignmentConfig::center("id2"),
            }],
        };

        let result = element_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_element_config_center_duplicate() {
        let data_elements = TemplateElements::from_ids(vec![String::from("id1")]);
        let validation_data = ValidationData {
            template_elements: &data_elements,
        };
        let element_config = ElementsConfig {
            elements: vec![
                ElementConfig {
                    id: String::from("id1"),
                    align: AlignmentConfig::default(),
                },
                ElementConfig {
                    id: String::from("id1"),
                    align: AlignmentConfig::default(),
                },
            ],
        };

        let result = element_config.validate(&validation_data);
        assert!(result.is_err());
    }
}
