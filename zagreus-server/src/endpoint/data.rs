use crate::data::asset::AssetSource;
use crate::endpoint::{send_instance_message, send_instance_message_with_condition};
use crate::websocket::message::ServerMessage;
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExecuteAnimationsWithStateDto {
    queue: String,
    default_animation: Option<String>,
    state_name: String,
    state_animations: Vec<ExecuteAnimationWithStateDto>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExecuteAnimationWithStateDto {
    name: String,
    state_value: String,
}

pub(crate) async fn set_text(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetTextDto>,
) -> impl IntoResponse {
    let message = ServerMessage::SetText {
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
    let message = ServerMessage::AddClass {
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
    let message = ServerMessage::RemoveClass {
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
    let message = ServerMessage::ExecuteAnimation {
        animation_sequence: &payload.name,
        queue_id: payload.queue.as_deref(),
    };
    send_instance_message(&instance, &server, message).await;
    StatusCode::OK
}

pub(crate) async fn execute_animations_with_state(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ExecuteAnimationsWithStateDto>,
) -> impl IntoResponse {
    if let Some(default_animation) = &payload.default_animation {
        let message = ServerMessage::ExecuteAnimation {
            animation_sequence: default_animation,
            queue_id: Some(&payload.queue),
        };
        send_instance_message_with_condition(&instance, &server, message, |state| {
            state.get_state(&payload.state_name).is_none()
        })
        .await;
    }
    for animation in payload.state_animations {
        let message = ServerMessage::ExecuteAnimation {
            animation_sequence: &animation.name,
            queue_id: Some(&payload.queue),
        };
        send_instance_message_with_condition(&instance, &server, message, |state| {
            state.get_state(&payload.state_name) == Some(&animation.state_value)
        })
        .await;
    }

    StatusCode::OK
}

pub(crate) async fn set_image_source(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetImageSourceDto>,
) -> impl IntoResponse {
    let message = ServerMessage::SetImageSource {
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
    let message = ServerMessage::SetCustomVariable {
        name: &payload.name,
        value: &payload.value,
    };
    send_instance_message(&instance, &server, message).await
}
