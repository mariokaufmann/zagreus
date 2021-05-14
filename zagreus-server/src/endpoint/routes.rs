use std::sync::Arc;

use futures::{StreamExt, TryStreamExt};
use warp::Buf;
use warp::Filter;

use crate::{ServerTemplateRegistry, ZAGREUS_VERSION};
use crate::config::ZagreusServerConfig;
use crate::controller::ServerController;
use crate::error::ZagreusError;
use crate::fs::get_templates_data_folder;
use crate::websocket::message::TemplateMessage;
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

// TODO return proper errors or replies
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs

pub fn get_routes(
    server_controller: Arc<ServerController>,
    ws_server: Arc<WebsocketServer>,
    template_registry: ServerTemplateRegistry,
    configuration: &ZagreusServerConfig,
) -> Result<impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone, ZagreusError>
{
    let websocket_server_data = warp::any().map(move || ws_server.clone());
    let template_registry_server_data = warp::any().map(move || template_registry.clone());
    let server_controller_server_data = warp::any().map(move || server_controller.clone());

    let ws_filter = warp::path("ws")
        .and(warp::path("template"))
        .and(warp::path::param())
        .and(warp::ws())
        .and(server_controller_server_data)
        .map(
            |template_name: String, ws: warp::ws::Ws, server_controller: Arc<ServerController>| {
                ws.on_upgrade(|socket| async move {
                    server_controller
                        .add_websocket_client(socket, &template_name)
                        .await;
                })
            },
        );

    // static files
    let templates_data_folder = get_templates_data_folder(&configuration.data_folder)?;
    let static_file_filter = warp::path("static");
    let template_file_filter = static_file_filter
        .and(warp::path("template"))
        .and(warp::fs::dir(templates_data_folder));
    const RUNTIME_FILE_NAME: &str = "zagreus-runtime.js";
    let runtime_file_filter = static_file_filter
        .and(warp::path(RUNTIME_FILE_NAME))
        .and(warp::path::end())
        .and(warp::fs::file(format!("./{}", RUNTIME_FILE_NAME)));
    const RUNTIME_SOURCE_MAP_FILE_NAME: &str = "zagreus-runtime.js.map";
    let runtime_source_map_file_filter = static_file_filter
        .and(warp::path(RUNTIME_SOURCE_MAP_FILE_NAME))
        .and(warp::path::end())
        .and(warp::fs::file(format!(
            "./{}",
            RUNTIME_SOURCE_MAP_FILE_NAME
        )));

    const SWAGGER_DOCS_FOLDER_NAME: &str = "swagger-docs";
    let swagger_docs_folder_filter = static_file_filter
        .and(warp::path(SWAGGER_DOCS_FOLDER_NAME))
        .and(warp::fs::dir(format!("./{}", SWAGGER_DOCS_FOLDER_NAME)));

    // REST API
    let api_filter = warp::path("api");

    let version_filter = api_filter
        .and(warp::path("version"))
        .and(warp::get())
        .and(warp::path::end())
        .map(|| ZAGREUS_VERSION);

    let template_filter = api_filter.and(warp::path("template"));
    let template_param_filter = template_filter.and(warp::path::param());
    let data_filter = template_param_filter.and(warp::path("data"));
    let text_filter = data_filter
        .and(warp::path("text"))
        .and(websocket_server_data.clone())
        .and(warp::body::json());
    let set_text_filter = text_filter.and(warp::post()).and_then(
        |template_name: String, server: Arc<WebsocketServer>, payload: SetTextDto| async move {
            let message = TemplateMessage::SetText {
                id: &payload.id,
                text: &payload.text,
            };
            send_template_message(&template_name, server, message).await
        },
    );
    let class_filter = data_filter.and(warp::path("class"));
    let add_class_filter =
        class_filter
            .and(warp::path("add"))
            .and(warp::body::json())
            .and(websocket_server_data.clone())
            .and(warp::post())
            .and_then(
                |template_name: String,
                 payload: ManipulateClassDto,
                 server: Arc<WebsocketServer>| async move {
                    let message = TemplateMessage::AddClass {
                        id: &payload.id,
                        class: &payload.class,
                    };
                    send_template_message(&template_name, server, message).await
                },
            );
    let remove_class_filter =
        class_filter
            .and(warp::path("remove"))
            .and(warp::body::json())
            .and(websocket_server_data.clone())
            .and(warp::post())
            .and_then(
                |template_name: String,
                 payload: ManipulateClassDto,
                 server: Arc<WebsocketServer>| async move {
                    info!("Removing class");
                    let message = TemplateMessage::RemoveClass {
                        id: &payload.id,
                        class: &payload.class,
                    };
                    send_template_message(&template_name, server, message).await
                },
            );
    let animation_filter = data_filter
        .and(warp::path("animation"))
        .and(websocket_server_data.clone())
        .and(warp::path::param());
    let execute_animation_filter = animation_filter.and(warp::post()).and_then(
        |template_name: String, server: Arc<WebsocketServer>, animation_name: String| async move {
            let message = TemplateMessage::ExecuteAnimation {
                animation_sequence: &animation_name,
            };
            send_template_message(&template_name, server, message).await
        },
    );

    let image_filter = data_filter
        .and(warp::path("image"))
        .and(websocket_server_data)
        .and(warp::body::json());
    let set_image_source_filter = image_filter
        .and(warp::post())
        .and_then(|template_name: String, server: Arc<WebsocketServer>, payload: SetImageSourceDto| async move {
            let message = TemplateMessage::SetImageSource { id: &payload.id, asset: &payload.asset };
            send_template_message(&template_name, server, message).await
        });

    let upload_template_filter = template_filter
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::post())
        .and(template_registry_server_data)
        .and(warp::multipart::form())
        .and_then(
            |template_name: String,
             template_registry: ServerTemplateRegistry,
             form: warp::multipart::FormData| {
                upload_template(template_name, template_registry, form)
            },
        );

    let filters = ws_filter
        .or(runtime_file_filter)
        .or(runtime_source_map_file_filter)
        .or(swagger_docs_folder_filter)
        .or(template_file_filter)
        .or(version_filter)
        .or(upload_template_filter)
        .or(set_text_filter)
        .or(add_class_filter)
        .or(remove_class_filter)
        .or(execute_animation_filter)
        .or(set_image_source_filter);

    Ok(filters)
}

async fn upload_template(
    template_name: String,
    template_registry: ServerTemplateRegistry,
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let mut parts: Vec<Result<warp::multipart::Part, warp::Error>> = form.collect().await;
    if parts.len() != 1 {
        error!("Multipart form data does not have expected form.");
        return Err(warp::reject::reject());
    }
    match parts.remove(0) {
        Ok(part) => {
            let name = part.name();
            if name.eq("template.zip") {
                let buffer = part
                    .stream()
                    .try_fold(Vec::<u8>::new(), |mut vec, data| {
                        vec.extend_from_slice(data.chunk());
                        futures::future::ready(Ok(vec))
                    })
                    .await;
                match buffer {
                    Ok(buffer) => {
                        let mut locked_registry = template_registry.write().await;
                        locked_registry.upload_packed_template(&template_name, buffer);
                    }
                    Err(err) => error!("Could not collect multipart file into buffer: {}.", err),
                }
            }
        }
        Err(err) => error!("Could not unpack multipart form data: {}.", err),
    }
    Ok(warp::reply())
}

async fn send_template_message(
    template_name: &str,
    server: Arc<WebsocketServer>,
    message: TemplateMessage<'_>,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    server
        .send_message_to_template_clients(&template_name, &message)
        .await;
    Ok(warp::reply())
}
