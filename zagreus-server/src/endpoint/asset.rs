use crate::error::ZagreusError;
use axum::body::Bytes;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::path::PathBuf;

pub const ASSETS_FOLDER_NAME: &str = "assets";

pub(crate) async fn get_asset_filenames(
    Extension(mut templates_data_folder): Extension<PathBuf>,
    Path(template_name): Path<String>,
) -> impl IntoResponse {
    if template_name.contains('.') || template_name.contains(std::path::MAIN_SEPARATOR) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!("Template name contains invalid character")),
        );
    }

    templates_data_folder.push(template_name);
    templates_data_folder.push(ASSETS_FOLDER_NAME);

    let template_assets_folder = templates_data_folder;

    match std::fs::read_dir(&template_assets_folder) {
        Ok(files) => {
            let entries: Vec<String> = files
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name())
                .map(|file_name| file_name.into_string())
                .filter_map(|file_name| file_name.ok())
                .collect();
            (StatusCode::OK, Json(json!(entries)))
        }
        Err(err) => {
            error!(
                "Could not read assets directory {}: {}.",
                &template_assets_folder.display(),
                err
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Could not read template assets.")),
            )
        }
    }
}
//
// const ASSET_NAME_FIELD: &str = "name";
// const ASSET_DATA_FIELD: &str = "file";
//
// pub(crate) async fn upload_asset(multipart: axum::extract::Multipart) {}
//
// async fn get_asset_data(
//     mut multipart: axum::extract::Multipart,
// ) -> Result<(String, Bytes), ZagreusError> {
//     let asset_name: Option<String> = None;
//     let asset_data: Option<Bytes> = None;
//     while let Some(field) = multipart.next_field().await? {
//         if let Some(name) = field.name() {
//             let data = field.bytes().await?;
//             if name.eq(ASSET_NAME_FIELD) {
//
//             }
//             let data = field.bytes().await?;
//             return Ok((name, data));
//         }
//     }
//     Err(ZagreusError::from(String::from(
//         "Multipart request did not have expected format.",
//     )))
// }
