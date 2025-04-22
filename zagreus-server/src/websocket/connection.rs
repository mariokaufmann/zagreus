use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::websocket::message::ServerMessage;

pub struct ClientState {
    states: HashMap<String, String>,
}

impl ClientState {
    pub fn get_state(&self, state: &str) -> Option<&String> {
        self.states.get(state)
    }

    pub fn set_state(&mut self, state: String, value: String) {
        self.states.insert(state, value);
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
            states: HashMap::new(),
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

    pub fn send_message(&self, message: &ServerMessage) {
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
