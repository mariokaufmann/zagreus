use crate::data::validation::{get_duplicate_elements, ConfigValidate, ValidationData};
use crate::error::{error_with_message, simple_error, ZagreusError};

#[derive(Serialize, Deserialize)]
pub struct ElementsConfig {
    elements: Vec<ElementConfig>,
}

impl ConfigValidate for ElementsConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        for element_config in &self.elements {
            if !validation_data
                .data_elements
                .has_data_element(&element_config.id)
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementConfig {
    id: String,
    align: AlignmentConfig,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
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

        if !self.with.is_empty() && !validation_data.data_elements.has_data_element(&self.with) {
            return Err(ZagreusError::from(format!(
                "Element is configured to be aligned with unknown element {}.",
                &self.with
            )));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize, PartialEq)]
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
    use crate::data::DataElements;

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
        let data_elements = DataElements::new(vec![String::from("id1"), String::from("id2")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
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
        let data_elements = DataElements::new(vec![String::from("id2")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
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
        let data_elements = DataElements::new(vec![String::from("id1")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
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
        let data_elements = DataElements::new(vec![String::from("id1")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
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
        let data_elements = DataElements::new(vec![String::from("id1")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
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
