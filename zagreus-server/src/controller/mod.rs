use std::borrow::Cow;
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedReceiver;

use crate::template::event::TemplateEvent;
use crate::template::Template;
use crate::websocket::message::TemplateMessage;
use crate::websocket::server::WebsocketServer;
use crate::ServerTemplateRegistry;

pub struct ServerController {
    websocket_server: Arc<WebsocketServer>,
    template_registry: ServerTemplateRegistry,
}

impl ServerController {
    pub fn new(
        template_event_receiver: UnboundedReceiver<TemplateEvent>,
        websocket_server: Arc<WebsocketServer>,
        template_registry: ServerTemplateRegistry,
    ) -> ServerController {
        // spawn template event handler
        tokio::spawn(Self::handle_template_events(
            template_event_receiver,
            websocket_server.clone(),
            template_registry.clone(),
        ));

        ServerController {
            websocket_server,
            template_registry,
        }
    }

    pub async fn add_websocket_client(&self, socket: warp::ws::WebSocket, template_name: &str) {
        self.websocket_server
            .add_client_socket(socket, template_name)
            .await;
        // (re-)send initial state for all clients of the template
        let locked_registry = self.template_registry.read().await;
        match locked_registry.get_template(template_name) {
            Some(template) => {
                let message = TemplateMessage::LoadAnimations {
                    animations: Cow::Borrowed(&template.animations.sequences),
                };
                self.websocket_server
                    .send_message_to_template_clients(template_name, &message)
                    .await;
                let message = TemplateMessage::LoadElements {
                    elements: Cow::Borrowed(&template.elements),
                };
                self.websocket_server
                    .send_message_to_template_clients(template_name, &message)
                    .await;
                let message = TemplateMessage::OnLoad {
                    animation_sequences: Cow::Borrowed(
                        &template.animations.on_load.animation_sequences,
                    ),
                };
                self.websocket_server
                    .send_message_to_template_clients(&template.name, &message)
                    .await;
            }
            None => error!(
                "Could not find template {} in registry when adding new websocket client.",
                template_name
            ),
        }
    }

    async fn handle_template_events(
        mut template_event_receiver: UnboundedReceiver<TemplateEvent>,
        server: Arc<WebsocketServer>,
        template_registry: ServerTemplateRegistry,
    ) {
        while let Some(template_event) = template_event_receiver.recv().await {
            match template_event {
                TemplateEvent::TemplateReloaded { template_name } => {
                    let locked_registry = template_registry.read().await;
                    match locked_registry.get_template(&template_name) {
                        Some(template) => {
                            Self::send_messages_on_template_update(template, &server).await;
                        }
                        None => error!(
                            "Received template event for template {} which is not in registry.",
                            &template_name
                        ),
                    }
                }
            }
        }
    }

    async fn send_messages_on_template_update(template: &Template, server: &Arc<WebsocketServer>) {
        // send element configs
        let message = TemplateMessage::LoadElements {
            elements: Cow::Borrowed(&template.elements),
        };
        server
            .send_message_to_template_clients(&template.name, &message)
            .await;

        // send new animations
        let message = TemplateMessage::LoadAnimations {
            animations: Cow::Borrowed(&template.animations.sequences),
        };
        server
            .send_message_to_template_clients(&template.name, &message)
            .await;

        // send on load config
        let message = TemplateMessage::OnLoad {
            animation_sequences: Cow::Borrowed(&template.animations.on_load.animation_sequences),
        };
        server
            .send_message_to_template_clients(&template.name, &message)
            .await;
    }
}
