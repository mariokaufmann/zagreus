use crate::ServerTemplateRegistry;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;

const MULTIPART_PART_NAME: &str = "template.zip";

pub(crate) async fn upload_template(
    Extension(template_registry): Extension<ServerTemplateRegistry>,
    Path(template_name): Path<String>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    // TODO unwraps
    if let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name.eq(MULTIPART_PART_NAME) {
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
    } else {
        (
            StatusCode::BAD_REQUEST,
            "Multipart form data does not have expected form.".to_owned(),
        )
    }
}
