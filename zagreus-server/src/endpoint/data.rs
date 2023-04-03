use crate::data::asset::AssetSource;
use crate::websocket::message::InstanceMessage;
use crate::WebsocketServer;
use axum::extract::{Extension, Path};
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

pub(crate) async fn set_text(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetTextDto>,
) -> impl IntoResponse {
    let message = InstanceMessage::SetText {
        id: &payload.id,
        text: &payload.text,
    };
    send_instance_message(&instance, server, message).await
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
    send_instance_message(&instance, server, message).await
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
    send_instance_message(&instance, server, message).await
}

pub(crate) async fn execute_animation(
    Path((instance, animation_name)): Path<(String, String)>,
    Extension(server): Extension<Arc<WebsocketServer>>,
) -> impl IntoResponse {
    let message = InstanceMessage::ExecuteAnimation {
        animation_sequence: &animation_name,
    };
    send_instance_message(&instance, server, message).await
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
    send_instance_message(&instance, server, message).await
}

async fn send_instance_message(
    instance: &str,
    server: Arc<WebsocketServer>,
    message: InstanceMessage<'_>,
) {
    server
        .send_message_to_instance_clients(instance, &message)
        .await
}
