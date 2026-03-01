use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum AssetSource {
    Zagreus,
    Template,
}
