use axum::Router;
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::http::uri::InvalidUri;
use axum::http::{Request, StatusCode, Uri};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::config::ZagreusServerConfig;
use crate::controller::ServerController;
use crate::endpoint;
use crate::endpoint::websocket::ws_handler;
use crate::fs::get_assets_folder;
use crate::websocket::server::WebsocketServer;

// e.g. rewrite /static/template/my-template to /static/template/my-template/
// TODO parse url better (what if there are multiple dots in the asset name?)
// TODO still necessary?
async fn map_rewrite_template_url(req: Request<Body>) -> Result<Request<Body>, StatusCode> {
    let uri = req.uri().to_string();
    if uri.starts_with("/static/template/") && !uri.ends_with('/') {
        let last_part = uri.split('/').next_back();

        if let Some(last_part) = last_part
            && !last_part.contains('.')
        {
            let (mut parts, body) = req.into_parts();
            let new_uri: Result<Uri, InvalidUri> = format!("{uri}/").parse();
            match new_uri {
                Ok(new_uri) => {
                    parts.uri = new_uri;
                    return Ok(Request::from_parts(parts, body));
                }
                Err(invalid_uri) => {
                    error!("URI was invalid: {}.", invalid_uri);
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }
    Ok(req)
}

pub fn get_router(
    configuration: &ZagreusServerConfig,
    ws_server: Arc<WebsocketServer>,
    server_controller: Arc<ServerController>,
) -> anyhow::Result<Router> {
    let assets_folder = get_assets_folder(&configuration.data_folder)?;
    let (api_router, openapi) = OpenApiRouter::with_openapi(endpoint::openapi::ApiDoc::openapi())
        .routes(routes!(crate::endpoint::get_server_version))
        .routes(routes!(crate::endpoint::asset::upload_asset))
        .routes(routes!(crate::endpoint::data::set_text))
        .routes(routes!(crate::endpoint::data::add_class))
        .routes(routes!(crate::endpoint::data::remove_class))
        .routes(routes!(crate::endpoint::data::execute_animation))
        .routes(routes!(crate::endpoint::data::set_image_source))
        .routes(routes!(crate::endpoint::data::set_custom_variable))
        .routes(routes!(crate::endpoint::state::get_state))
        .routes(routes!(crate::endpoint::state::set_state))
        .layer(axum::extract::Extension(ws_server))
        .layer(axum::extract::Extension(assets_folder.clone()))
        .split_for_parts();

    let mut router = Router::new()
        .merge(api_router)
        .route(
            "/api/openapi.json",
            axum::routing::get(endpoint::openapi::get_openapi_json),
        )
        .route(
            "/api/openapi.yaml",
            axum::routing::get(endpoint::openapi::get_openapi_yaml),
        )
        .layer(axum::extract::Extension(openapi));

    let assets_router = Router::new().nest_service(
        "/assets",
        axum::routing::get_service(ServeDir::new(&assets_folder)).handle_error(|err| async move {
            error!("error occurred when serving assets: {}.", err)
        }),
    );
    router = router.merge(assets_router);

    let static_router = Router::new().nest(
        "/static",
        Router::new()
            .route(
                "/zagreus-runtime.js",
                axum::routing::get_service(tower_http::services::ServeFile::new(
                    "zagreus-runtime.js",
                ))
                .handle_error(|err| async move {
                    error!("error occurred when serving zagreus runtime: {}.", err)
                }),
            )
            .route(
                "/zagreus-runtime.js.map",
                axum::routing::get_service(tower_http::services::ServeFile::new(
                    "zagreus-runtime.js.map",
                ))
                .handle_error(|err| async move {
                    error!(
                        "error occurred when serving zagreus runtime source map: {}.",
                        err
                    )
                }),
            )
            .nest_service(
                "/swagger-docs",
                axum::routing::get_service(tower_http::services::ServeDir::new("swagger-docs"))
                    .handle_error(|err| async move {
                        error!("error occurred when serving swagger docs: {}.", err)
                    }),
            ),
    );
    router = router.merge(static_router);

    // route for websocket router
    let websocket_router = Router::new()
        .route("/ws/instance/{instance}", axum::routing::get(ws_handler))
        .layer(axum::extract::Extension(server_controller));
    router = router.merge(websocket_router);

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {error}"),
            )
        }))
        .layer(axum::middleware::map_request(map_rewrite_template_url));

    Ok(router.layer(middleware_stack))
}
