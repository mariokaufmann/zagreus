#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AssetSource {
    Zagreus,
    Template,
}
