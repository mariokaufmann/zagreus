use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use futures::stream::SplitStream;
use futures::FutureExt;
use futures::StreamExt;
use tokio::sync::RwLock;

use crate::websocket::connection::WebsocketConnection;
use crate::websocket::message::InstanceMessage;
use tokio_stream::wrappers::UnboundedReceiverStream;

type UserConnections =
    Arc<RwLock<HashMap<usize, crate::websocket::connection::WebsocketConnection>>>;

pub struct WebsocketServer {
    next_user_id: AtomicUsize,
    connections: UserConnections,
}

impl WebsocketServer {
    pub fn new() -> WebsocketServer {
        WebsocketServer {
            connections: Arc::new(RwLock::new(HashMap::new())),
            next_user_id: AtomicUsize::new(0),
        }
    }

    pub async fn add_client_socket(
        &self,
        websocket: axum::extract::ws::WebSocket,
        template_name: &str,
    ) {
        let id = self.next_user_id.fetch_add(1, Ordering::SeqCst);
        info!(
            "Connected to new websocket client with id {} and template {}.",
            id, template_name
        );

        let (websocket_sink, websocket_stream) = websocket.split();

        // sending
        let (sender_tx, sender_rx) = tokio::sync::mpsc::unbounded_channel();
        let sender_rx = UnboundedReceiverStream::new(sender_rx);
        let sending_stream =
            tokio_stream::StreamExt::take_while(sender_rx, |result| match result {
                Ok(_) => true,
                Err(err) => {
                    error!("Could not forward message to websocket sink: {}.", err);
                    false
                }
            });
        tokio::task::spawn(sending_stream.forward(websocket_sink).map(|result| {
            if let Err(err) = result {
                error!("Could not send message on websocket: {}.", err);
            }
        }));

        let connection = WebsocketConnection::new(sender_tx, String::from(template_name));
        self.connections.write().await.insert(id, connection);

        // user messages and disconnect handler
        tokio::spawn(Self::handle_user_messages(
            id,
            websocket_stream,
            self.connections.clone(),
        ));
    }

    async fn handle_user_messages(
        id: usize,
        mut stream: SplitStream<axum::extract::ws::WebSocket>,
        connections: UserConnections,
    ) {
        loop {
            match stream.next().await {
                Some(message_result) => match message_result {
                    Ok(message) => {
                        match serde_json::from_slice::<InstanceMessage>(&message.into_data()) {
                            Ok(parsed_message) => {
                                if let InstanceMessage::LogError { message, stack } = parsed_message
                                {
                                    error!("Template error occurred: {}\n{}", message, stack)
                                }
                            }
                            Err(err) => error!("Could not parse message on websocket: {}.", err),
                        }
                    }
                    Err(err) => {
                        error!("Could not receive message for client: {}.", err);
                        break;
                    }
                },
                None => {
                    warn!("Could not await new message on websocket.");
                    break;
                }
            }
        }

        // as soon as the loop quits the client has disconnected
        Self::user_disconnected(&connections, id).await;
    }

    async fn user_disconnected(connections: &UserConnections, id: usize) {
        debug!("Client with id {} has disconnected.", id);
        connections.write().await.remove(&id);
    }

    pub async fn send_message_to_instance_clients(
        &self,
        instance: &str,
        message: &InstanceMessage<'_>,
    ) {
        let locked_connections = self.connections.read().await;
        let connection_entries = locked_connections.values();

        for connection in connection_entries {
            if connection.is_from_instance(instance) {
                connection.send_message(message);
            }
        }
    }
}
