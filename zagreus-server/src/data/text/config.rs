#[derive(Serialize, Deserialize, Clone)]
pub struct TextConfig {
    pub elements: Vec<TextElementConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextElementConfig {
    id: String,
    align: TextAlignment,
    align_with: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TextAlignment {
    Center,
    Left,
    Right,
}
