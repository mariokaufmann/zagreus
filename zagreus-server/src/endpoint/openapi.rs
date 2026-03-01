use axum::Extension;
use axum::http::header;
use axum::response::IntoResponse;

#[derive(utoipa::OpenApi)]
#[openapi(
    tags(
        (name = "General", description = "General API endpoints"),
        (name = "Asset", description = "Asset management"),
        (name = "Data", description = "Template data manipulation"),
        (name = "State", description = "State management")
    ),
    info(
        title = "Zagreus Server API",
        description = "HTTP API for the Zagreus server.",
        license(name = "MIT", url = "https://github.com/mariokaufmann/zagreus/blob/main/LICENSE")
    )
)]
pub struct ApiDoc;

pub async fn get_openapi_json(
    Extension(openapi): Extension<utoipa::openapi::OpenApi>,
) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/json")],
        openapi
            .to_pretty_json()
            .unwrap_or_else(|_| "{}".to_string()),
    )
}

pub async fn get_openapi_yaml(
    Extension(openapi): Extension<utoipa::openapi::OpenApi>,
) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/yaml")],
        openapi.to_yaml().unwrap_or_else(|_| String::new()),
    )
}
