use std::path::PathBuf;

pub mod loader;

const DEFAULT_DATA_FOLDER: &str = "data";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZagreusServerConfig {
    pub data_folder: PathBuf,
}

impl Default for ZagreusServerConfig {
    fn default() -> Self {
        let folder = match crate::fs::get_application_folder(crate::APPLICATION_NAME) {
            Ok(mut folder) => {
                folder.push(DEFAULT_DATA_FOLDER);

                if !folder.exists() {
                    match std::fs::create_dir_all(&folder) {
                        Ok(()) => folder,
                        Err(err) => {
                            error!("Could not create folder {:?}: {}.", &folder, err);
                            PathBuf::new()
                        }
                    }
                } else {
                    folder
                }
            }
            Err(err) => {
                error!("Could not get application folder to create default config: {}.", err);
                PathBuf::new()
            }
        };
        ZagreusServerConfig { data_folder: folder }
    }
}
