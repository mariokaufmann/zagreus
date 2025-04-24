use crate::endpoint::send_instance_message;
use crate::websocket::message::ServerMessage;
use crate::websocket::server::WebsocketServer;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Deserialize)]
pub(crate) struct GetStateQueryParams {
    name: String,
}

pub(crate) async fn get_state(
    Path(instance): Path<String>,
    params: Query<GetStateQueryParams>,
    Extension(server): Extension<Arc<WebsocketServer>>,
) -> impl IntoResponse {
    let state_values = server
        .get_client_states(&instance, |state| {
            state.get_state(&params.name).map(|val| val.to_string())
        })
        .await
        .into_iter()
        .flatten()
        .collect::<HashSet<String>>();
    Json(state_values)
}

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
    send_instance_message(&instance, &server, message).await;
    StatusCode::OK
}
