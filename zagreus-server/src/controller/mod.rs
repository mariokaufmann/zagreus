use std::sync::Arc;

use crate::websocket::server::WebsocketServer;

pub struct ServerController {
    websocket_server: Arc<WebsocketServer>,
}

impl ServerController {
    pub fn new(websocket_server: Arc<WebsocketServer>) -> ServerController {
        ServerController { websocket_server }
    }

    pub async fn add_websocket_client(&self, socket: axum::extract::ws::WebSocket, instance: &str) {
        self.websocket_server
            .add_client_socket(socket, instance)
            .await
    }
}
