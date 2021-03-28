#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimationConfig {
    pub on_load: OnLoadConfig,
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
    iterations: String,
    direction: AnimationDirection,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnLoadConfig {
    pub animation_sequences: Vec<String>,
}
