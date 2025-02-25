use crate::data::asset::AssetSource;
use crate::websocket::connection::ClientState;
use crate::websocket::message::InstanceMessage;
use crate::WebsocketServer;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub(crate) struct SetTextDto {
    id: String,
    text: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ManipulateClassDto {
    id: String,
    class: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetImageSourceDto {
    id: String,
    asset: String,
    asset_source: AssetSource,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetCustomVariableDto {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExecuteAnimationDto {
    name: String,
    queue: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExecuteAnimationTransitionDto {
    from: String,
    to: Option<String>,
    default: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExecuteAnimationTransitionsDto {
    queue: String,
    transitions: Vec<ExecuteAnimationTransitionDto>,
}

pub(crate) async fn set_text(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetTextDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::SetText {
        id: &payload.id,
        text: &payload.text,
    };
    send_instance_message(&instance, &server, message).await
}

pub(crate) async fn add_class(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ManipulateClassDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::AddClass {
        id: &payload.id,
        class: &payload.class,
    };
    send_instance_message(&instance, &server, message).await
}

pub(crate) async fn remove_class(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ManipulateClassDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::RemoveClass {
        id: &payload.id,
        class: &payload.class,
    };
    send_instance_message(&instance, &server, message).await
}

pub(crate) async fn execute_animation(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ExecuteAnimationDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::ExecuteAnimation {
        animation_sequence: &payload.name,
        queue_id: payload.queue.as_deref(),
    };
    send_instance_message(&instance, &server, message).await;
    StatusCode::OK
}

pub(crate) async fn execute_animation_transition(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ExecuteAnimationTransitionsDto>,
) -> impl IntoResponse {
    for transition in payload.transitions {
        if let Some(to) = transition.to {
            let message = InstanceMessage::ExecuteAnimation {
                animation_sequence: &to,
                queue_id: Some(&payload.queue),
            };

            if transition.default.unwrap_or(false) {
                send_instance_message(&instance, &server, message).await;
            } else {
                send_instance_message_with_condition(&instance, &server, message, |state| {
                    state.is_last_executed_animation_in_queue(&payload.queue, &transition.from)
                })
                .await;
            }
        }
    }

    StatusCode::OK
}

pub(crate) async fn set_image_source(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetImageSourceDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::SetImageSource {
        id: &payload.id,
        asset: &payload.asset,
        asset_source: payload.asset_source,
    };
    send_instance_message(&instance, &server, message).await
}

pub(crate) async fn set_custom_variable(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetCustomVariableDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::SetCustomVariable {
        name: &payload.name,
        value: &payload.value,
    };
    send_instance_message(&instance, &server, message).await
}

async fn send_instance_message(
    instance: &str,
    server: &WebsocketServer,
    message: InstanceMessage<'_>,
) {
    server
        .send_message_to_instance_clients(instance, &message)
        .await
}

async fn send_instance_message_with_condition<F>(
    instance: &str,
    server: &WebsocketServer,
    message: InstanceMessage<'_>,
    condition: F,
) where
    F: Fn(&ClientState) -> bool,
{
    server
        .send_message_to_instance_clients_with_condition(instance, &message, condition)
        .await
}
