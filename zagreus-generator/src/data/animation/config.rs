use crate::data::validation::{get_duplicate_elements, ValidationData};
use crate::data::ConfigValidate;
use crate::error::ZagreusError;

#[derive(Serialize, Deserialize)]
pub struct AnimationConfig {
    sequences: Vec<AnimationSequence>,
}

impl AnimationConfig {
    pub fn with_defaults() -> Self {
        AnimationConfig { sequences: vec![] }
    }
}

impl ConfigValidate for AnimationConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        for sequence in &self.sequences {
            for step in &sequence.steps {
                // check for duplicate animations on the same element
                let duplicate_elements =
                    get_duplicate_elements(&step.animations, |animation| &animation.id);

                if let Some(duplicate_element) = duplicate_elements.get(0) {
                    return Err(ZagreusError::from(
                        format!("Animation sequence {} contains multiple animations for element {} in the same step.", &sequence.name, duplicate_element)));
                }

                for animation in &step.animations {
                    if !validation_data
                        .data_elements
                        .has_data_element(&animation.id)
                    {
                        return Err(ZagreusError::from(format!(
                            "Animation config contains unknown element {}.",
                            &animation.id
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnimationSequence {
    name: String,
    steps: Vec<AnimationStep>,
}

#[derive(Serialize, Deserialize)]
pub struct AnimationStep {
    #[serde(default)]
    start: u16,
    duration: u16,
    animations: Vec<Animation>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    id: String,
    name: String,
    #[serde(default)]
    direction: AnimationDirection,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimationDirection {
    Normal,
    Reverse,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        AnimationDirection::Normal
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataElements;

    use super::*;

    #[test]
    fn validate_animation_config_valid() {
        let data_elements = DataElements::new(vec![String::from("id1"), String::from("id2")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let animation_config = AnimationConfig {
            sequences: vec![AnimationSequence {
                name: String::from("sequence"),
                steps: vec![AnimationStep {
                    start: 0,
                    duration: 0,
                    animations: vec![
                        Animation {
                            id: String::from("id1"),
                            name: String::from("ani1"),
                            direction: AnimationDirection::Normal,
                        },
                        Animation {
                            id: String::from("id2"),
                            name: String::from("ani2"),
                            direction: AnimationDirection::Normal,
                        },
                    ],
                }],
            }],
        };

        let result = animation_config.validate(&validation_data);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_animation_config_inexistant_element() {
        let data_elements = DataElements::new(vec![String::from("id1")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let animation_config = AnimationConfig {
            sequences: vec![AnimationSequence {
                name: String::from("sequence"),
                steps: vec![AnimationStep {
                    start: 0,
                    duration: 0,
                    animations: vec![Animation {
                        id: String::from("id2"),
                        name: String::from("ani2"),
                        direction: AnimationDirection::Normal,
                    }],
                }],
            }],
        };

        let result = animation_config.validate(&validation_data);
        assert!(result.is_err());
    }

    #[test]
    fn validate_animation_config_duplicate() {
        let data_elements = DataElements::new(vec![String::from("id1")]);
        let validation_data = ValidationData {
            data_elements: &data_elements,
        };
        let animation_config = AnimationConfig {
            sequences: vec![AnimationSequence {
                name: String::from("sequence"),
                steps: vec![AnimationStep {
                    start: 0,
                    duration: 0,
                    animations: vec![
                        Animation {
                            id: String::from("id1"),
                            name: String::from("ani1"),
                            direction: AnimationDirection::Normal,
                        },
                        Animation {
                            id: String::from("id1"),
                            name: String::from("ani2"),
                            direction: AnimationDirection::Normal,
                        },
                    ],
                }],
            }],
        };

        let result = animation_config.validate(&validation_data);
        assert!(result.is_err());
    }
}
