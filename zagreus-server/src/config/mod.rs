use std::path::PathBuf;

pub mod loader;

const DEFAULT_DATA_FOLDER: &str = "data";
const DEFAULT_SERVER_PORT: u16 = 58180;

fn get_default_data_folder() -> PathBuf {
    match crate::fs::get_application_folder(crate::APPLICATION_NAME) {
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
            error!(
                "Could not get application folder to create default config: {}.",
                err
            );
            PathBuf::new()
        }
    }
}

fn get_default_server_port() -> u16 {
    DEFAULT_SERVER_PORT
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZagreusServerConfig {
    #[serde(default = "get_default_server_port")]
    pub server_port: u16,
    #[serde(default = "get_default_data_folder")]
    pub data_folder: PathBuf,
}

impl Default for ZagreusServerConfig {
    fn default() -> Self {
        ZagreusServerConfig {
            server_port: get_default_server_port(),
            data_folder: get_default_data_folder(),
        }
    }
}
