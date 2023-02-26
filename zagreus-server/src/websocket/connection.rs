use tokio::sync::mpsc::UnboundedSender;

use crate::websocket::message::InstanceMessage;

pub struct WebsocketConnection {
    message_sender: UnboundedSender<Result<axum::extract::ws::Message, axum::Error>>,
    instance: String,
}

impl WebsocketConnection {
    pub fn new(
        message_sender: UnboundedSender<Result<axum::extract::ws::Message, axum::Error>>,
        instance: String,
    ) -> WebsocketConnection {
        WebsocketConnection {
            message_sender,
            instance,
        }
    }

    pub fn is_from_instance(&self, instance: &str) -> bool {
        self.instance.eq(instance)
    }

    pub fn send_message(&self, message: &InstanceMessage) {
        match serde_json::to_string(message) {
            Ok(serialized_message) => {
                let ws_message = axum::extract::ws::Message::Text(serialized_message);
                if let Err(err) = self.message_sender.send(Ok(ws_message)) {
                    error!("Could not send websocket message on channel: {}.", err);
                }
            }
            Err(err) => error!("Could not serialize message: {}.", err),
        }
    }
}
