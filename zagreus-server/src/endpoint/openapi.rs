use axum::Extension;
use axum::http::header;
use axum::response::IntoResponse;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::endpoint::get_server_version,
        crate::endpoint::asset::upload_asset,
        crate::endpoint::data::set_text,
        crate::endpoint::data::add_class,
        crate::endpoint::data::remove_class,
        crate::endpoint::data::execute_animation,
        crate::endpoint::data::set_image_source,
        crate::endpoint::data::set_custom_variable,
        crate::endpoint::state::get_state,
        crate::endpoint::state::set_state
    ),
    components(schemas(
        crate::data::asset::AssetSource,
        crate::endpoint::asset::UploadAssetRequestDto,
        crate::endpoint::asset::UploadAssetResponseDto,
        crate::endpoint::data::SetTextDto,
        crate::endpoint::data::ManipulateClassDto,
        crate::endpoint::data::ExecuteAnimationDto,
        crate::endpoint::data::SetImageSourceDto,
        crate::endpoint::data::SetCustomVariableDto,
        crate::endpoint::state::SetStateDto,
        crate::endpoint::state::GetStateDto,
        crate::endpoint::state::GetStateItemDto
    )),
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
