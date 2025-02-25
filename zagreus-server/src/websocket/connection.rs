use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::websocket::message::InstanceMessage;

pub struct ClientState {
    last_executed_animations: HashMap<String, String>,
}

impl ClientState {
    pub fn is_last_executed_animation_in_queue(&self, queue: &str, animation: &str) -> bool {
        match self.last_executed_animations.get(queue) {
            Some(last) => animation.eq(last),
            // if no animation was executed in that queue we just return true
            None => true,
        }
    }

    pub fn set_last_executed_animation_in_queue(&mut self, queue: String, animation: String) {
        self.last_executed_animations.insert(queue, animation);
    }
}

pub struct WebsocketConnection {
    message_sender: UnboundedSender<Result<axum::extract::ws::Message, axum::Error>>,
    instance: String,
    client_state: ClientState,
}

impl WebsocketConnection {
    pub fn new(
        message_sender: UnboundedSender<Result<axum::extract::ws::Message, axum::Error>>,
        instance: String,
    ) -> WebsocketConnection {
        let client_state = ClientState {
            last_executed_animations: HashMap::new(),
        };
        WebsocketConnection {
            message_sender,
            instance,
            client_state,
        }
    }

    pub fn is_from_instance(&self, instance: &str) -> bool {
        self.instance.eq(instance)
    }

    pub fn get_client_state(&self) -> &ClientState {
        &self.client_state
    }

    pub fn get_mut_client_state(&mut self) -> &mut ClientState {
        &mut self.client_state
    }

    pub fn send_message(&self, message: &InstanceMessage) {
        match serde_json::to_string(message) {
            Ok(serialized_message) => {
                let ws_message = axum::extract::ws::Message::Text(serialized_message.into());
                if let Err(err) = self.message_sender.send(Ok(ws_message)) {
                    error!("Could not send websocket message on channel: {}.", err);
                }
            }
            Err(err) => error!("Could not serialize message: {}.", err),
        }
    }
}
