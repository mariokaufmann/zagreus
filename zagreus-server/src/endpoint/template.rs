use crate::error::ZagreusError;
use crate::ServerTemplateRegistry;
use axum::body::Bytes;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;

const MULTIPART_PART_NAME: &str = "template.zip";

pub(crate) async fn upload_template(
    Extension(template_registry): Extension<ServerTemplateRegistry>,
    Path(template_name): Path<String>,
    multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    match get_template_data(multipart).await {
        Ok((field_name, data)) => {
            if field_name.eq(MULTIPART_PART_NAME) {
                let mut locked_registry = template_registry.write().await;
                locked_registry.upload_packed_template(&template_name, &data);
                (
                    StatusCode::CREATED,
                    "Uploaded template successfully.".to_owned(),
                )
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Multipart file name was different from expected value: {}",
                        MULTIPART_PART_NAME
                    ),
                )
            }
        }
        Err(err) => {
            error!("Could not upload template to server: {}.", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse upload request.".to_owned(),
            )
        }
    }
}

async fn get_template_data(
    mut multipart: axum::extract::Multipart,
) -> Result<(String, Bytes), ZagreusError> {
    if let Some(field) = multipart.next_field().await? {
        if let Some(name) = field.name().map(|name| name.to_owned()) {
            let data = field.bytes().await?;
            return Ok((name, data));
        }
    }
    Err(ZagreusError::from(String::from(
        "Multipart request did not have expected format.",
    )))
}
