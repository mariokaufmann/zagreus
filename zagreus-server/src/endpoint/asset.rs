use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};

use axum::body::Bytes;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::error::ZagreusError;

pub const ASSETS_FOLDER_NAME: &str = "assets";

pub(crate) async fn get_asset_filenames(
    Extension(mut templates_data_folder): Extension<PathBuf>,
    axum::extract::Path(template_name): axum::extract::Path<String>,
) -> impl IntoResponse {
    if !is_valid_template_name(&template_name) {
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
            let entries: Vec<String> = list_recursively(files)
                .iter()
                .map(|entry| entry.file_name())
                .map(|filename| filename.into_string())
                .filter_map(|filename| filename.ok())
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

fn list_recursively(files: ReadDir) -> Vec<DirEntry> {
    files
        .filter_map(|entry| entry.ok())
        .flat_map(|entry| {
            if is_file(&entry) {
                vec![entry]
            } else {
                list_recursively(std::fs::read_dir(entry.path()).unwrap())
            }
        })
        .collect()
}

fn is_file(dir_entry: &DirEntry) -> bool {
    match dir_entry.metadata() {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

const ASSET_NAME_FIELD: &str = "name";
const ASSET_DATA_FIELD: &str = "file";

pub(crate) async fn upload_asset(
    Extension(templates_data_folder): Extension<PathBuf>,
    axum::extract::Path(template_name): axum::extract::Path<String>,
    multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    if !is_valid_template_name(&template_name) {
        return (
            StatusCode::BAD_REQUEST,
            "Template name contains invalid character",
        );
    }

    match get_asset_data(multipart).await {
        Ok((asset_name, asset_data)) => {
            if asset_name.contains(std::path::MAIN_SEPARATOR) || asset_name.contains("..") {
                return (
                    StatusCode::BAD_REQUEST,
                    "Asset name contains invalid character",
                );
            }

            match write_asset_file(
                &templates_data_folder,
                &template_name,
                &asset_name,
                asset_data,
            )
            .await
            {
                Ok(()) => (StatusCode::OK, "Asset uploaded successfully."),
                Err(err) => {
                    error!("Could not upload asset successfully: {}.", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Could not upload asset.")
                }
            }
        }
        Err(err) => {
            error!("Could not upload asset to server: {}.", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse upload request.",
            )
        }
    }
}

async fn get_asset_data(
    mut multipart: axum::extract::Multipart,
) -> Result<(String, Bytes), ZagreusError> {
    let mut asset_name: Option<String> = None;
    let mut asset_data: Option<Bytes> = None;
    while let Some(field) = multipart.next_field().await? {
        if let Some(name) = field.name() {
            if name.eq(ASSET_NAME_FIELD) {
                let asset_name_text = field.text().await?;
                asset_name = Some(asset_name_text);
            } else if name.eq(ASSET_DATA_FIELD) {
                let data = field.bytes().await?;
                asset_data = Some(data);
            }
        }
    }
    if let Some(asset_name) = asset_name {
        if let Some(asset_data) = asset_data {
            return Ok((asset_name, asset_data));
        }
    }
    Err(ZagreusError::from(String::from(
        "Multipart request did not have expected format.",
    )))
}

async fn write_asset_file(
    templates_data_folder: &Path,
    template_name: &str,
    asset_name: &str,
    asset_bytes: Bytes,
) -> Result<(), ZagreusError> {
    let mut asset_file_path = templates_data_folder.to_owned();
    asset_file_path.push(template_name);
    asset_file_path.push(ASSETS_FOLDER_NAME);
    asset_file_path.push(asset_name);

    tokio::fs::write(asset_file_path, asset_bytes).await?;

    Ok(())
}

fn is_valid_template_name(template_name: &str) -> bool {
    !template_name.contains('.') && !template_name.contains(std::path::MAIN_SEPARATOR)
}
