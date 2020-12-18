use crate::data::ConfigValidate;
use crate::data::validation::{get_duplicate_elements, ValidationData};
use crate::error::ZagreusError;

#[derive(Serialize, Deserialize)]
pub struct AnimationConfig {
    sequences: Vec<AnimationSequence>,
}

impl ConfigValidate for AnimationConfig {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError> {
        for sequence in &self.sequences {
            for step in &sequence.steps {
                // check for duplicate animations on the same element
                let duplicate_elements = get_duplicate_elements(&step.animations, |animation| &animation.id);
                for duplicate_element in duplicate_elements {
                    return Err(ZagreusError::from(
                        format!("Animation sequence {} contains multiple animations for element {} in the same step.", &sequence.name, duplicate_element)));
                }

                for animation in &step.animations {
                    if !validation_data.data_elements.has_data_element(&animation.id) {
                        return Err(ZagreusError::from(format!("Animation config contains unknown element {}.", &animation.id)));
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