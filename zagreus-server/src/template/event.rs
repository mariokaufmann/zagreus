#[derive(Serialize, Deserialize)]
#[serde(tag = "tag", content = "payload")]
pub enum TemplateEvent {
    TemplateReloaded { template_name: String }
}