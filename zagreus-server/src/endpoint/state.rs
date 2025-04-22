use crate::endpoint::send_instance_message;
use crate::websocket::message::ServerMessage;
use crate::websocket::server::WebsocketServer;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetStateDto {
    name: String,
    value: String,
}

pub(crate) async fn set_state(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetStateDto>,
) -> impl IntoResponse {
    let message = ServerMessage::SetState {
        name: &payload.name,
        value: &payload.value,
    };
    send_instance_message(&instance, &server, message).await
}
