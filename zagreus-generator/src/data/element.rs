use crate::data::validation::{ConfigValidate, get_duplicate_elements, ValidationData};
use crate::error::{error_with_message, ZagreusError};

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
                    "Text config contains unknown element {}.",
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
        if let Some(duplicate_element) = duplicate_elements.get(0) {
            return Err(ZagreusError::from(format!(
                "Text element {} is configured more than once.",
                duplicate_element
            )));
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentConfig {
    #[serde(default = "Alignment::default_horizontal")]
    horizontal: Alignment,
    #[serde(default = "Alignment::default_vertical")]
    vertical: Alignment,
    #[serde(default)]
    with: String,
}

impl AlignmentConfig {
    fn center(align_with: &str) -> AlignmentConfig {
        AlignmentConfig {
            horizontal: Alignment::Center,
            vertical: Alignment::Center,
            with: align_with.to_owned(),
        }
    }
}

impl Default for AlignmentConfig {
    fn default() -> Self {
        AlignmentConfig {
            horizontal: Alignment::Left,
            vertical: Alignment::Top,
            with: Default::default(),
        }
    }
}

impl ConfigValidate for AlignmentConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        if self.horizontal == Alignment::Center && self.with.is_empty() {
            return Err(ZagreusError::from(format!(
                "Element is center-aligned but no alignWith is configured."
            )));
        }

        if !self.with.is_empty()
            && !validation_data
                .data_elements
                .has_data_element(&self.with)
        {
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
pub enum Alignment {
    Center,
    Left,
    Right,
    Top,
    Bottom,
}

impl Alignment {
    fn default_horizontal() -> Self {
        Alignment::Left
    }

    fn default_vertical() -> Self {
        Alignment::Top
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataElements;
    use crate::data::validation::ValidationData;

    use super::*;

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
    fn validate_element_config_inexistant_element() {
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
