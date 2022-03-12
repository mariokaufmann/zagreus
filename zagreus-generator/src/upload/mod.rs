use std::io::Read;
use std::path::Path;
use std::time::Duration;

pub struct TemplateUploader<'a> {
    client: reqwest::blocking::Client,
    upload_url: String,
    packed_template_file_path: &'a Path,
}

impl<'a> TemplateUploader<'a> {
    pub fn new(
        server_url: &str,
        template_name: &str,
        packed_template_file_path: &'a Path,
    ) -> anyhow::Result<TemplateUploader<'a>> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        let upload_url = format!("http://{}/api/template/{}", server_url, template_name);

        Ok(TemplateUploader {
            client,
            upload_url,
            packed_template_file_path,
        })
    }

    pub fn upload_template(&self) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        let mut input_file = std::fs::File::open(self.packed_template_file_path)?;
        input_file.read_to_end(&mut buffer)?;
        let part = reqwest::blocking::multipart::Part::bytes(buffer);
        let multipart = reqwest::blocking::multipart::Form::new().part("template.zip", part);
        let response = self
            .client
            .post(&self.upload_url)
            .multipart(multipart)
            .send()?;

        if !response.status().is_success() {
            error!("Error response when uploading template: {:?}.", response);
        }

        Ok(())
    }
}
