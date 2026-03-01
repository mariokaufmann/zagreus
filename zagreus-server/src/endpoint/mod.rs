use crate::ZAGREUS_VERSION;
use axum::response::IntoResponse;

mod asset;
mod data;
pub mod openapi;
pub mod routes;
mod state;
mod websocket;

#[utoipa::path(
    get,
    path = "/api/version",
    tag = "General",
    summary = "Get server version",
    responses(
        (status = 200, description = "Server version", body = String, content_type = "text/plain"),
    )
)]
async fn get_server_version() -> impl IntoResponse {
    ZAGREUS_VERSION
}
