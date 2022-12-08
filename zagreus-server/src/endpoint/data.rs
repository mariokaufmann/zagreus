use crate::websocket::message::TemplateMessage;
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
pub(crate) struct SetImageSourceDto {
    id: String,
    asset: String,
}

pub(crate) async fn set_text(
    Path(template_name): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetTextDto>,
) -> impl IntoResponse {
    let message = TemplateMessage::SetText {
        id: &payload.id,
        text: &payload.text,
    };
    send_template_message(&template_name, server, message).await
}

pub(crate) async fn add_class(
    Path(template_name): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ManipulateClassDto>,
) -> impl IntoResponse {
    let message = TemplateMessage::AddClass {
        id: &payload.id,
        class: &payload.class,
    };
    send_template_message(&template_name, server, message).await
}

pub(crate) async fn remove_class(
    Path(template_name): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ManipulateClassDto>,
) -> impl IntoResponse {
    let message = TemplateMessage::RemoveClass {
        id: &payload.id,
        class: &payload.class,
    };
    send_template_message(&template_name, server, message).await
}

pub(crate) async fn execute_animation(
    Path((template_name, animation_name)): Path<(String, String)>,
    Extension(server): Extension<Arc<WebsocketServer>>,
) -> impl IntoResponse {
    let message = TemplateMessage::ExecuteAnimation {
        animation_sequence: &animation_name,
    };
    send_template_message(&template_name, server, message).await
}

pub(crate) async fn set_image_source(
    Path(template_name): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetImageSourceDto>,
) -> impl IntoResponse {
    let message = TemplateMessage::SetImageSource {
        id: &payload.id,
        asset: &payload.asset,
    };
    send_template_message(&template_name, server, message).await
}

async fn send_template_message(
    template_name: &str,
    server: Arc<WebsocketServer>,
    message: TemplateMessage<'_>,
) {
    server
        .send_message_to_template_clients(template_name, &message)
        .await
}
