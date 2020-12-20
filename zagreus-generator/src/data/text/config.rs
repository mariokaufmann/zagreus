use crate::data::ConfigValidate;
use crate::data::validation::{get_duplicate_elements, ValidationData};
use crate::error::ZagreusError;

#[derive(Serialize, Deserialize)]
pub struct TextConfig {
    elements: Vec<TextElementConfig>,
}

impl ConfigValidate for TextConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        for text_element_config in &self.elements {
            if !validation_data.data_elements.has_data_element(&text_element_config.id) {
                return Err(ZagreusError::from(format!("Text config contains unknown element {}.", &text_element_config.id)));
            }

            // check alignment
            if text_element_config.align == TextAlignment::Center && text_element_config.align_with.is_empty() {
                return Err(ZagreusError::from(format!("Text element {} is center-aligned but no alignWith is configured.", &text_element_config.id)));
            }

            if !text_element_config.align_with.is_empty() && !validation_data.data_elements.has_data_element(&text_element_config.align_with) {
                return Err(ZagreusError::from(
                    format!("Text element {} is configured to be aligned with unknown element {}.", &text_element_config.id, &text_element_config.align_with)));
            }
        }

        // check for duplicate elements
        let duplicate_elements = get_duplicate_elements(&self.elements, |element| &element.id);
        if let Some(duplicate_element) = duplicate_elements.get(0) {
            return Err(ZagreusError::from(
                format!("Text element {} is configured more than once.", duplicate_element)));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextElementConfig {
    id: String,
    #[serde(default)]
    align: TextAlignment,
    #[serde(default)]
    align_with: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextAlignment {
    Center,
    Left,
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Left
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataElements;

    use super::*;

    #[test]
    fn validate_text_config_valid() {
        let data_elements = DataElements::new(vec![
            String::from("id1"),
            String::from("id2"),
        ]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let text_config = TextConfig {
            elements: vec![
                TextElementConfig { id: String::from("id1"), align: TextAlignment::Left, align_with: String::from("") },
                TextElementConfig { id: String::from("id2"), align: TextAlignment::Center, align_with: String::from("id1") },
            ]
        };

        let result = text_config.validate(&validation_data);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_text_config_inexistant_element() {
        let data_elements = DataElements::new(vec![
            String::from("id2"),
        ]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let text_config = TextConfig {
            elements: vec![
                TextElementConfig { id: String::from("id1"), align: TextAlignment::Left, align_with: String::from("") },
            ]
        };

        let result = text_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_text_config_center_no_align_with() {
        let data_elements = DataElements::new(vec![
            String::from("id1"),
        ]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let text_config = TextConfig {
            elements: vec![
                TextElementConfig { id: String::from("id1"), align: TextAlignment::Center, align_with: String::from("") },
            ]
        };

        let result = text_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_text_config_center_invalid_align_with() {
        let data_elements = DataElements::new(vec![
            String::from("id1"),
        ]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let text_config = TextConfig {
            elements: vec![
                TextElementConfig { id: String::from("id1"), align: TextAlignment::Center, align_with: String::from("id2") },
            ]
        };

        let result = text_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_text_config_center_duplicate() {
        let data_elements = DataElements::new(vec![
            String::from("id1"),
        ]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let text_config = TextConfig {
            elements: vec![
                TextElementConfig { id: String::from("id1"), align: TextAlignment::Left, align_with: String::from("") },
                TextElementConfig { id: String::from("id1"), align: TextAlignment::Left, align_with: String::from("") },
            ]
        };

        let result = text_config.validate(&validation_data);
        assert!(result.is_err());
    }
}
