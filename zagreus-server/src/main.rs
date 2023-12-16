#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use anyhow::Context;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::cli::{get_command, ZagreusServerCommand};
use crate::config::loader::ConfigurationManager;
use crate::config::ZagreusServerConfig;
use crate::controller::ServerController;
use crate::websocket::server::WebsocketServer;

mod cli;
mod config;
mod controller;
mod data;
mod endpoint;
mod fs;
mod logger;
mod websocket;

const ZAGREUS_VERSION: &str = env!("CARGO_PKG_VERSION");

const APPLICATION_NAME: &str = "zagreus-server";
const CONFIG_FILE_NAME: &str = "config.json";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = get_command();
    let application_folder =
        fs::get_application_folder(APPLICATION_NAME).context("Could not get application folder")?;
    logger::init_logger(command.verbose);

    let manager =
        ConfigurationManager::<ZagreusServerConfig>::load(&application_folder, CONFIG_FILE_NAME)
            .context("Could not load configuration")?;
    let mut configuration = manager.get_configuration();
    override_configuration_with_cli_flags(&mut configuration, command);
    start_with_config(configuration).await
}

async fn start_with_config(configuration: ZagreusServerConfig) -> anyhow::Result<()> {
    info!("Starting zagreus server...");
    let server_port = configuration.server_port;
    info!(
        "API docs are available at http://localhost:{}/static/swagger-docs/?url=spec.yaml",
        server_port
    );
    let ws_server = Arc::new(WebsocketServer::new());

    let server_controller = Arc::new(ServerController::new(ws_server.clone()));

    let router =
        endpoint::routes::get_router(&configuration, ws_server.clone(), server_controller.clone())?;
    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .await
        .context("Could not run web server")?;

    Ok(())
}

fn override_configuration_with_cli_flags(
    configuration: &mut ZagreusServerConfig,
    command: ZagreusServerCommand,
) {
    if let Some(data_folder) = command.data_folder {
        configuration.data_folder = data_folder;
    }

    if let Some(server_port) = command.server_port {
        configuration.server_port = server_port;
    }
}
