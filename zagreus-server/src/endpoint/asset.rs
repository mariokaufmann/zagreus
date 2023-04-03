use std::path::{Path, PathBuf};

use anyhow::anyhow;
use axum::body::Bytes;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;

#[derive(Deserialize, Serialize)]
pub(crate) struct UploadAssetResponseDto {
    name: String,
}

const ASSET_NAME_FIELD: &str = "name";
const ASSET_DATA_FIELD: &str = "file";

pub(crate) async fn upload_asset(
    Extension(assets_folder): Extension<PathBuf>,
    multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    match get_asset_data(multipart).await {
        Ok((asset_name, asset_data)) => {
            if asset_name.contains(std::path::MAIN_SEPARATOR) || asset_name.contains("..") {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!("Asset name contains invalid character")),
                );
            }

            let path = PathBuf::from(asset_name);

            match path.extension().and_then(|val| val.to_str()) {
                Some(extension) => {
                    match write_asset_file(&assets_folder, extension, asset_data).await {
                        Ok(asset_name) => (
                            StatusCode::OK,
                            Json(json!(UploadAssetResponseDto { name: asset_name })),
                        ),
                        Err(err) => {
                            error!("Could not upload asset successfully: {}.", err);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!("Could not upload asset.")),
                            )
                        }
                    }
                }
                None => {
                    error!("Could not parse asset name");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!("Invalid asset name.")),
                    )
                }
            }
        }
        Err(err) => {
            error!("Could not upload asset to server: {}.", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Could not parse upload request.")),
            )
        }
    }
}

async fn get_asset_data(
    mut multipart: axum::extract::Multipart,
) -> anyhow::Result<(String, Bytes)> {
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
    Err(anyhow!("Multipart request did not have expected format."))
}

async fn write_asset_file(
    assets_folder: &Path,
    extension: &str,
    asset_bytes: Bytes,
) -> anyhow::Result<String> {
    let hash = Sha256::digest(&asset_bytes);
    let saved_asset_name = format!("{:x}.{extension}", hash);

    let mut asset_file_path = assets_folder.to_owned();
    asset_file_path.push(&saved_asset_name);

    tokio::fs::write(asset_file_path, asset_bytes).await?;

    Ok(saved_asset_name)
}
