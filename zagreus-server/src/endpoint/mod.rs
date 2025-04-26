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
