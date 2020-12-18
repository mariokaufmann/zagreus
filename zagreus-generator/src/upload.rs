use std::io::Read;
use std::time::Duration;

use crate::error::ZagreusError;
use crate::get_path_in_build_folder;

pub struct TemplateUploader<'a> {
    client: reqwest::blocking::Client,
    upload_url: String,
    packed_template_file_name: &'a str,
}

impl<'a> TemplateUploader<'a> {
    pub fn new(server_url: &str, template_name: &str, packed_template_file_name: &'a str) -> Result<TemplateUploader<'a>, ZagreusError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        let upload_url = format!("http://{}/api/template/{}", server_url, template_name);

        Ok(TemplateUploader {
            client,
            upload_url,
            packed_template_file_name,
        })
    }

    pub fn upload_template(&self) -> Result<(), ZagreusError> {
        let packed_template_file_name = get_path_in_build_folder(self.packed_template_file_name);
        let mut buffer = Vec::new();
        let mut input_file = std::fs::File::open(packed_template_file_name)?;
        input_file.read_to_end(&mut buffer)?;
        let part = reqwest::blocking::multipart::Part::bytes(buffer);
        let multipart = reqwest::blocking::multipart::Form::new()
            .part("template.zip", part);
        let response = self.client
            .post(&self.upload_url)
            .multipart(multipart)
            .send()?;

        if !response.status().is_success() {
            error!("Error response when uploading template: {:?}.", response);
        }

        Ok(())
    }
}