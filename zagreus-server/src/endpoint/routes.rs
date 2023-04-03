use axum::error_handling::HandleErrorLayer;
use axum::http::uri::InvalidUri;
use axum::http::{Request, StatusCode, Uri};
use axum::Router;
use hyper::Body;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::config::ZagreusServerConfig;
use crate::controller::ServerController;
use crate::endpoint;
use crate::endpoint::websocket::ws_handler;
use crate::endpoint::{data, get_server_version};
use crate::fs::get_assets_folder;
use crate::websocket::server::WebsocketServer;

#[derive(Deserialize, Serialize)]
struct SetTextDto {
    id: String,
    text: String,
}

#[derive(Deserialize, Serialize)]
struct ManipulateClassDto {
    id: String,
    class: String,
}

#[derive(Deserialize, Serialize)]
struct SetImageSourceDto {
    id: String,
    asset: String,
}

// e.g. rewrite /static/template/my-template to /static/template/my-template/
// TODO parse url better (what if there are multiple dots in the asset name?)
// TODO still necessary?
async fn map_rewrite_template_url(req: Request<Body>) -> Result<Request<Body>, StatusCode> {
    let uri = req.uri().to_string();
    if uri.starts_with("/static/template/") && !uri.ends_with('/') {
        let last_part = uri.split('/').last();

        if let Some(last_part) = last_part {
            if !last_part.contains('.') {
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
    }
    Ok(req)
}

pub fn get_router(
    configuration: &ZagreusServerConfig,
    ws_server: Arc<WebsocketServer>,
    server_controller: Arc<ServerController>,
) -> anyhow::Result<Router> {
    let mut router = Router::new().route("/api/version", axum::routing::get(get_server_version));

    let assets_folder = get_assets_folder(&configuration.data_folder)?;
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
        .route("/ws/instance/:instance", axum::routing::get(ws_handler))
        .layer(axum::extract::Extension(server_controller));
    router = router.merge(websocket_router);

    // routes for manipulating template instances
    let manipulate_templates_router = Router::new().nest(
        "/api/instance/:instance",
        Router::new()
            .route("/data/text", axum::routing::post(data::set_text))
            .route("/data/class/add", axum::routing::post(data::add_class))
            .route(
                "/data/class/remove",
                axum::routing::post(data::remove_class),
            )
            .route(
                "/data/animation/:animation_name",
                axum::routing::post(data::execute_animation),
            )
            .route("/data/image", axum::routing::post(data::set_image_source))
            .layer(axum::extract::Extension(ws_server)),
    );
    router = router.merge(manipulate_templates_router);

    // route for manipulating assets
    let assets_router = Router::new()
        .route(
            "/api/asset",
            axum::routing::post(endpoint::asset::upload_asset),
        )
        .layer(axum::extract::Extension(assets_folder));
    router = router.merge(assets_router);

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
