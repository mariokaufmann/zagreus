use crate::websocket::message::ServerMessage;
use crate::websocket::server::WebsocketServer;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetStateDto {
    clients_unset: Vec<usize>,
    clients_set: Vec<GetStateItemDto>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetStateItemDto {
    clients: HashSet<usize>,
    value: String,
}

#[derive(Deserialize)]
pub(crate) struct GetStateQueryParams {
    name: String,
}

pub(crate) async fn get_state(
    Path(instance): Path<String>,
    params: Query<GetStateQueryParams>,
    Extension(server): Extension<Arc<WebsocketServer>>,
) -> impl IntoResponse {
    let mut grouped_clients: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut clients_unset = Vec::new();
    server
        .iterate_client_states(&instance, |state| {
            if let Some(state_value) = state.get_state(&params.name) {
                grouped_clients
                    .entry(state_value.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(state.client_id);
            } else {
                clients_unset.push(state.client_id);
            }
        })
        .await;
    let client_set_items = grouped_clients
        .into_iter()
        .map(|(state_value, clients)| GetStateItemDto {
            clients,
            value: state_value,
        })
        .collect::<Vec<GetStateItemDto>>();
    Json(GetStateDto {
        clients_unset,
        clients_set: client_set_items,
    })
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetStateDto {
    name: String,
    value: Option<String>,
}

pub(crate) async fn set_state(
    Path(instance): Path<String>,
    Extension(server): Extension<Arc<WebsocketServer>>,
    Json(payload): Json<SetStateDto>,
) -> impl IntoResponse {
    let message = ServerMessage::SetState {
        name: &payload.name,
        value: payload.value.as_ref().map(|v| v.as_str()),
    };
    server
        .send_message_to_instance_clients(&instance, &message)
        .await;
    StatusCode::OK
}
