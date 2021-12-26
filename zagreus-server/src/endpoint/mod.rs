use crate::ZAGREUS_VERSION;
use axum::response::IntoResponse;

pub mod data;
pub mod routes;
pub mod template;
pub mod websocket;

async fn get_server_version() -> impl IntoResponse {
    ZAGREUS_VERSION
}
