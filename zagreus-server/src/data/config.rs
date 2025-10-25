#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateElement {
    id: String,
    config: Option<ElementConfig>,
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
