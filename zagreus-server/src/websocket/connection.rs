use tokio::sync::mpsc::UnboundedSender;

use crate::websocket::message::TemplateMessage;

pub struct WebsocketConnection {
    message_sender: UnboundedSender<Result<warp::ws::Message, warp::Error>>,
    template_name: String,
}

impl WebsocketConnection {
    pub fn new(message_sender: UnboundedSender<Result<warp::ws::Message, warp::Error>>, template_name: String) -> WebsocketConnection {
        WebsocketConnection { message_sender, template_name }
    }

    pub fn is_from_template(&self, template_name: &str) -> bool {
        self.template_name.eq(template_name)
    }

    pub fn send_message(&self, message: &TemplateMessage) {
        match serde_json::to_string(message) {
            Ok(serialized_message) => {
                let ws_message = warp::ws::Message::text(serialized_message);
                if let Err(err) = self.message_sender.send(Ok(ws_message)) {
                    error!("Could not send websocket message on channel: {}.", err);
                }
            }
            Err(err) => error!("Could not serialize message: {}.", err),
        }
    }
}