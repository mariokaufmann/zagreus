#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::cli::{get_command, ZagreusServerCommand};
use crate::config::loader::ConfigurationManager;
use crate::config::ZagreusServerConfig;
use crate::controller::ServerController;
use crate::template::registry::TemplateRegistry;
use crate::websocket::server::WebsocketServer;

mod cli;
mod config;
mod controller;
mod data;
mod endpoint;
mod error;
mod fs;
mod logger;
mod template;
mod websocket;

const ZAGREUS_VERSION: &str = env!("CARGO_PKG_VERSION");

const APPLICATION_NAME: &str = "zagreus-server";
const CONFIG_FILE_NAME: &str = "config.json";

type ServerTemplateRegistry = Arc<tokio::sync::RwLock<TemplateRegistry>>;

#[tokio::main]
async fn main() {
    let command = get_command();
    let application_folder = fs::get_application_folder(APPLICATION_NAME).unwrap_or_else(|err| {
        panic!("Could not get application folder: {}", err);
    });
    logger::init_logger(command.verbose);

    match ConfigurationManager::<ZagreusServerConfig>::load(&application_folder, CONFIG_FILE_NAME) {
        Ok(manager) => {
            let mut configuration = manager.get_configuration();
            override_configuration_with_cli_flags(&mut configuration, command);
            start_with_config(configuration).await
        }
        Err(err) => error!("Could not load configuration: {}.", err),
    }
}

async fn start_with_config(configuration: ZagreusServerConfig) {
    info!("Starting zagreus server...");
    let server_port = configuration.server_port;
    info!(
        "API docs are available at http://localhost:{}/static/swagger-docs/?url=spec.yaml",
        server_port
    );
    let ws_server = Arc::new(WebsocketServer::new());
    let (template_event_tx, template_event_rx) = tokio::sync::mpsc::unbounded_channel();
    let mut template_registry =
        TemplateRegistry::new(&configuration.data_folder, template_event_tx);
    template_registry.load_templates();
    let template_registry = Arc::new(tokio::sync::RwLock::new(template_registry));

    let server_controller = Arc::new(ServerController::new(
        template_event_rx,
        ws_server.clone(),
        template_registry.clone(),
    ));

    match endpoint::routes::get_router(
        &configuration,
        ws_server.clone(),
        server_controller.clone(),
        template_registry.clone(),
    ) {
        Ok(router) => {
            let addr = SocketAddr::from(([0, 0, 0, 0], server_port));
            if let Err(err) = axum_server::bind(addr)
                .serve(router.into_make_service())
                .await
            {
                error!("Could not start server: {}", err);
            }
        }
        Err(err) => error!("Could not configure server routes: {}", err),
    }
}

fn override_configuration_with_cli_flags(
    configuration: &mut ZagreusServerConfig,
    command: ZagreusServerCommand,
) {
    if let Some(data_folder) = command.data_folder {
        configuration.data_folder = data_folder.clone();
    }

    if let Some(server_port) = command.server_port {
        configuration.server_port = server_port;
    }
}
