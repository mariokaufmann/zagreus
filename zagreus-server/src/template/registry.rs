use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tokio::sync::mpsc::UnboundedSender;

use crate::fs::get_templates_data_folder;
use crate::template::event::TemplateEvent;
use crate::template::Template;

// TODO send event when template is updated
#[allow(dead_code)]
pub struct TemplateRegistry {
    data_folder: PathBuf,
    templates: HashMap<String, Template>,
    template_event_sender: UnboundedSender<TemplateEvent>,
}

impl TemplateRegistry {
    pub fn new(data_folder: &Path, template_event_sender: UnboundedSender<TemplateEvent>) -> TemplateRegistry {
        TemplateRegistry { data_folder: data_folder.to_path_buf(), templates: HashMap::new(), template_event_sender }
    }

    pub fn load_templates(&mut self) {
        self.templates.clear();

        match get_templates_data_folder(&self.data_folder) {
            Ok(templates_folder) => {
                match std::fs::read_dir(&templates_folder) {
                    Ok(read_dir) => {
                        for entry in read_dir {
                            match entry {
                                Ok(entry) => {
                                    let path = entry.path();
                                    self.load_template_folder(&path);
                                }
                                Err(err) => error!("Error while trying to get directory entry of {}: {}.", self.data_folder.display(), err),
                            }
                        }
                    }
                    Err(err) => {
                        error!("Could not list directory entries for {}: {}.", self.data_folder.display(), err);
                    }
                }
            }
            Err(err) => error!("Could not get templates folder. Cannot load templates: {}.", err),
        }
    }

    fn load_template_folder(&mut self, template_folder: &Path) {
        if template_folder.is_dir() {
            debug!("Trying to load template from directory {}.", template_folder.display());
            match template_folder.file_name() {
                Some(file_name) => {
                    match file_name.to_str() {
                        Some(template_name) => self.load_template(template_name),
                        None => warn!("Could not use directory name as template name. Is it UTF-8?"),
                    }
                }
                None => warn!("Could not get directory name for {}.", template_folder.display()),
            }
        }
    }

    fn load_template(&mut self, template_name: &str) {
        match crate::template::Template::load(&self.data_folder, template_name) {
            Ok(template) => {
                match self.templates.insert(String::from(template_name), template) {
                    Some(_) => info!("Replaced template {} with new version.", template_name),
                    None => info!("Loaded new template {}.", template_name),
                }
            }
            Err(err) => {
                warn!("Could not load template {}: {}.", template_name, err);
            }
        }
    }

    pub fn get_template(&self, template_name: &str) -> Option<&Template> {
        self.templates.get(template_name)
    }

    pub fn upload_packed_template(&mut self, template_name: &str, packed_buffer: Vec<u8>) {
        match crate::fs::zip::unpack_template_files(template_name, packed_buffer, &self.data_folder) {
            Ok(()) => {
                debug!("Unpacked template {}.", template_name);
                self.load_template(template_name);
            }
            Err(err) => error!("Could not unpack template {}: {}.", template_name, err),
        }
    }
}