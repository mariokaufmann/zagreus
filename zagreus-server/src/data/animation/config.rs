#[derive(Serialize, Deserialize)]
pub struct AnimationConfig {
    pub sequences: Vec<AnimationSequence>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationSequence {
    name: String,
    steps: Vec<AnimationStep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationStep {
    start: u16,
    duration: u16,
    animations: Vec<Animation>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    id: String,
    name: String,
    direction: AnimationDirection,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AnimationDirection {
    Normal,
    Reverse,
}
