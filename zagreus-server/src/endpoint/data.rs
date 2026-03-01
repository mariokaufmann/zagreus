use crate::WebsocketServer;
use crate::data::asset::AssetSource;
use crate::websocket::message::ServerMessage;
use axum::Json;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub(crate) struct SetTextDto {
    id: String,
    text: String,
    client: Option<usize>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub(crate) struct ManipulateClassDto {
    id: String,
    class: String,
    client: Option<usize>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetImageSourceDto {
    id: String,
    asset: String,
    asset_source: AssetSource,
    client: Option<usize>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetCustomVariableDto {
    name: String,
    value: String,
    client: Option<usize>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExecuteAnimationDto {
    name: String,
    queue: Option<String>,
    client: Option<usize>,
}

#[utoipa::path(
    post,
    path = "/api/instance/{instance}/data/text",
    tag = "Data",
    summary = "Set text content",
    params(
        ("instance" = String, Path, description = "Template instance name")
    ),
    request_body = SetTextDto,
    responses(
        (status = 200, description = "Text updated"),
    )
)]
pub(crate) async fn set_text(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetTextDto>,
) -> impl IntoResponse {
    let message = ServerMessage::SetText {
        id: &payload.id,
        text: &payload.text,
    };
    send_message_with_optional_client(&instance, &server, message, payload.client).await;
    StatusCode::OK
}

#[utoipa::path(
    post,
    path = "/api/instance/{instance}/data/class/add",
    tag = "Data",
    summary = "Add CSS class",
    params(
        ("instance" = String, Path, description = "Template instance name")
    ),
    request_body = ManipulateClassDto,
    responses(
        (status = 200, description = "Class added"),
    )
)]
pub(crate) async fn add_class(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ManipulateClassDto>,
) -> impl IntoResponse {
    let message = ServerMessage::AddClass {
        id: &payload.id,
        class: &payload.class,
    };
    send_message_with_optional_client(&instance, &server, message, payload.client).await;
    StatusCode::OK
}

#[utoipa::path(
    post,
    path = "/api/instance/{instance}/data/class/remove",
    tag = "Data",
    summary = "Remove CSS class",
    params(
        ("instance" = String, Path, description = "Template instance name")
    ),
    request_body = ManipulateClassDto,
    responses(
        (status = 200, description = "Class removed"),
    )
)]
pub(crate) async fn remove_class(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ManipulateClassDto>,
) -> impl IntoResponse {
    let message = ServerMessage::RemoveClass {
        id: &payload.id,
        class: &payload.class,
    };
    send_message_with_optional_client(&instance, &server, message, payload.client).await;
    StatusCode::OK
}

#[utoipa::path(
    post,
    path = "/api/instance/{instance}/data/animation",
    tag = "Data",
    summary = "Execute animation",
    params(
        ("instance" = String, Path, description = "Template instance name")
    ),
    request_body = ExecuteAnimationDto,
    responses(
        (status = 200, description = "Animation command sent"),
    )
)]
pub(crate) async fn execute_animation(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<ExecuteAnimationDto>,
) -> impl IntoResponse {
    let message = ServerMessage::ExecuteAnimation {
        animation_sequence: &payload.name,
        queue_id: payload.queue.as_deref(),
    };
    send_message_with_optional_client(&instance, &server, message, payload.client).await;
    StatusCode::OK
}

#[utoipa::path(
    post,
    path = "/api/instance/{instance}/data/image",
    tag = "Data",
    summary = "Set image source",
    params(
        ("instance" = String, Path, description = "Template instance name")
    ),
    request_body = SetImageSourceDto,
    responses(
        (status = 200, description = "Image source updated"),
    )
)]
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
    send_message_with_optional_client(&instance, &server, message, payload.client).await;
    StatusCode::OK
}

#[utoipa::path(
    post,
    path = "/api/instance/{instance}/data/custom-variable",
    tag = "Data",
    summary = "Set CSS custom variable",
    params(
        ("instance" = String, Path, description = "Template instance name")
    ),
    request_body = SetCustomVariableDto,
    responses(
        (status = 200, description = "Custom variable updated"),
    )
)]
pub(crate) async fn set_custom_variable(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetCustomVariableDto>,
) -> impl IntoResponse {
    let message = ServerMessage::SetCustomVariable {
        name: &payload.name,
        value: &payload.value,
    };
    send_message_with_optional_client(&instance, &server, message, payload.client).await;
    StatusCode::OK
}

async fn send_message_with_optional_client(
    instance: &str,
    server: &WebsocketServer,
    message: ServerMessage<'_>,
    client: Option<usize>,
) {
    if let Some(client_id) = client {
        server
            .send_message_to_instance_client(instance, client_id, &message)
            .await;
    } else {
        server
            .send_message_to_instance_clients(instance, &message)
            .await;
    }
}
