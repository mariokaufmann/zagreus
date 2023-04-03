use crate::ServerController;
use axum::extract::ws::WebSocket;
use axum::extract::{Extension, Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(instance): Path<String>,
    Extension(server_controller): Extension<Arc<ServerController>>,
) -> impl IntoResponse {
    ws.on_upgrade(|websocket| handle_socket(websocket, server_controller, instance))
}

pub async fn handle_socket(
    socket: WebSocket,
    server_controller: Arc<ServerController>,
    instance: String,
) {
    server_controller
        .add_websocket_client(socket, &instance)
        .await;
}
