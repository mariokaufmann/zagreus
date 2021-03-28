use std::io::Read;
use std::path::{Path, PathBuf};

use crate::error::ZagreusError;

pub struct ConfigurationManager<T>
where
    T: Sized,
{
    configuration: T,
}

impl<T> ConfigurationManager<T>
where
    T: Default + serde::Serialize + serde::de::DeserializeOwned,
{
    pub fn load(
        application_folder: &Path,
        config_file_name: &str,
    ) -> Result<ConfigurationManager<T>, ZagreusError> {
        let configuration_loader = ConfigurationLoader::new(&application_folder, config_file_name);

        let configuration;
        if configuration_loader.config_exists() {
            configuration = configuration_loader.load_config()?;
        } else {
            configuration = T::default();
            configuration_loader.store_config(&configuration)?;
        }
        Ok(ConfigurationManager { configuration })
    }

    pub fn get_configuration(&self) -> &T {
        &self.configuration
    }
}

struct ConfigurationLoader {
    config_file_path: PathBuf,
}

impl ConfigurationLoader {
    pub fn new(application_folder: &Path, config_file_name: &str) -> ConfigurationLoader {
        let mut config_file_path = application_folder.to_owned();
        config_file_path.push(config_file_name);
        ConfigurationLoader { config_file_path }
    }

    pub fn load_config<T>(&self) -> Result<T, ZagreusError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut file = std::fs::File::open(&self.config_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn config_exists(&self) -> bool {
        self.config_file_path.exists()
    }

    pub fn store_config<T>(&self, config: &T) -> Result<(), ZagreusError>
    where
        T: serde::Serialize,
    {
        let serialized_data = serde_json::to_string_pretty(config)?;
        std::fs::write(&self.config_file_path, serialized_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_STRING_VALUE: &str = "This is the default value";
    const CONFIG_FILE_NAME: &str = "config.json";

    #[derive(Serialize, Deserialize, Clone)]
    struct TestConfig {
        string_value: String,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            TestConfig {
                string_value: DEFAULT_STRING_VALUE.to_owned(),
            }
        }
    }

    #[test]
    fn test_load_config_not_existing() {
        let path = crate::fs::temp::prepare_temp_folder().unwrap();
        let loader = ConfigurationLoader::new(&path, CONFIG_FILE_NAME);

        assert!(!loader.config_exists());

        let result = loader.load_config::<TestConfig>();
        assert!(result.is_err());
        crate::fs::temp::delete_temp_folder(&path).unwrap();
    }

    #[test]
    fn test_store_config() {
        let path = crate::fs::temp::prepare_temp_folder().unwrap();
        let loader = ConfigurationLoader::new(&path, CONFIG_FILE_NAME);

        const TEST_VALUE: &str = "Test value";
        let config = TestConfig {
            string_value: TEST_VALUE.to_owned(),
        };
        loader.store_config(&config).unwrap();

        let mut config_file_path = path.clone();
        config_file_path.push(CONFIG_FILE_NAME);

        assert!(config_file_path.exists());
        crate::fs::temp::delete_temp_folder(&path).unwrap();
    }

    #[test]
    fn test_store_and_load_config() {
        let path = crate::fs::temp::prepare_temp_folder().unwrap();
        let loader = ConfigurationLoader::new(&path, CONFIG_FILE_NAME);

        const TEST_VALUE: &str = "This is the expected text.";
        let config = TestConfig {
            string_value: TEST_VALUE.to_owned(),
        };
        loader.store_config(&config).unwrap();

        let loaded_config = loader.load_config::<TestConfig>().unwrap();

        assert_eq!(config.string_value, loaded_config.string_value);

        crate::fs::temp::delete_temp_folder(&path).unwrap();
    }

    #[test]
    fn test_create_config_manager() {
        let path = crate::fs::temp::prepare_temp_folder().unwrap();
        let manager = ConfigurationManager::<TestConfig>::load(&path, CONFIG_FILE_NAME).unwrap();

        assert_eq!(DEFAULT_STRING_VALUE, manager.configuration.string_value);

        crate::fs::temp::delete_temp_folder(&path).unwrap();
    }
}
