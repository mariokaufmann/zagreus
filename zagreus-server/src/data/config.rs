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
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
    with: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlignment {
    Center,
    Top,
    Bottom,
}
