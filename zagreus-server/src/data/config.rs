#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    pub on_load: OnLoadConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnLoadConfig {
    pub animation_sequences: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ElementConfigs {
    pub elements: Vec<ElementConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ElementConfig {
    id: String,
    align: AlignmentConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlignmentConfig {
    horizontal: Alignment,
    vertical: Alignment,
    with: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Center,
    Left,
    Right,
    Top,
    Bottom,
}
