use std::borrow::Cow;

use crate::data::animation::config::AnimationSequence;
use crate::data::config::ElementConfig;

#[derive(Serialize, Deserialize)]
#[serde(tag = "tag", content = "payload")]
pub enum TemplateMessage<'a> {
    SetText {
        id: &'a str,
        text: &'a str,
    },
    LogError {
        message: String,
        stack: String,
    },
    AddClass {
        id: &'a str,
        class: &'a str,
    },
    RemoveClass {
        id: &'a str,
        class: &'a str,
    },
    LoadAnimations {
        #[serde(borrow)]
        animations: Cow<'a, Vec<AnimationSequence>>,
    },
    #[serde(rename_all = "camelCase")]
    LoadElements {
        #[serde(borrow)]
        elements: Cow<'a, Vec<ElementConfig>>,
    },
    #[serde(rename_all = "camelCase")]
    ExecuteAnimation {
        animation_sequence: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    OnLoad {
        #[serde(borrow)]
        animation_sequences: Cow<'a, Vec<String>>,
    },
    SetImageSource {
        id: &'a str,
        asset: &'a str,
    },
}
