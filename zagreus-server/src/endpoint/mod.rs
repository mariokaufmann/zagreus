use crate::websocket::connection::ClientState;
use crate::websocket::message::ServerMessage;
use crate::websocket::server::WebsocketServer;
use crate::ZAGREUS_VERSION;
use axum::response::IntoResponse;

mod asset;
mod data;
pub mod routes;
mod state;
mod websocket;

async fn get_server_version() -> impl IntoResponse {
    ZAGREUS_VERSION
}

async fn send_instance_message(
    instance: &str,
    server: &WebsocketServer,
    message: ServerMessage<'_>,
) {
    server
        .send_message_to_instance_clients(instance, &message)
        .await
}

async fn send_instance_message_with_condition<F>(
    instance: &str,
    server: &WebsocketServer,
    message: ServerMessage<'_>,
    condition: F,
) where
    F: Fn(&ClientState) -> bool,
{
    server
        .send_message_to_instance_clients_with_condition(instance, &message, condition)
        .await
}
