use std::borrow::Cow;

use crate::data::animation::config::AnimationSequence;
use crate::data::asset::AssetSource;
use crate::data::config::TemplateElement;

#[derive(Serialize, Deserialize)]
#[serde(tag = "tag", content = "payload")]
pub enum ServerMessage<'a> {
    SetText {
        id: &'a str,
        text: &'a str,
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
        animations: Cow<'a, [AnimationSequence]>,
    },
    #[serde(rename_all = "camelCase")]
    LoadElements {
        #[serde(borrow)]
        elements: Cow<'a, [TemplateElement]>,
    },
    #[serde(rename_all = "camelCase")]
    ExecuteAnimation {
        animation_sequence: &'a str,
        queue_id: Option<&'a str>,
    },
    #[serde(rename_all = "camelCase")]
    OnLoad {
        #[serde(borrow)]
        animation_sequences: Cow<'a, [String]>,
    },
    #[serde(rename_all = "camelCase")]
    SetImageSource {
        id: &'a str,
        asset: &'a str,
        asset_source: AssetSource,
    },
    #[serde(rename_all = "camelCase")]
    SetCustomVariable {
        name: &'a str,
        value: &'a str,
    },
    SetState {
        name: &'a str,
        value: Option<&'a str>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "tag", content = "payload")]
pub enum ClientMessage {
    // Owned, a borrowed str cannot hold a value with JSON escapes.
    #[serde(rename_all = "camelCase")]
    StateSet {
        name: String,
        value: Option<String>,
    },
    LogError {
        message: String,
        stack: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_state_set_with_escaped_value() {
        let message =
            r#"{"tag":"StateSet","payload":{"name":"PERSON","value":"{\"name\":\"Jane\"}"}}"#;
        match serde_json::from_str::<ClientMessage>(message).unwrap() {
            ClientMessage::StateSet { name, value } => {
                assert_eq!(name, "PERSON");
                assert_eq!(value.as_deref(), Some(r#"{"name":"Jane"}"#));
            }
            ClientMessage::LogError { .. } => panic!("expected StateSet"),
        }
    }
}
