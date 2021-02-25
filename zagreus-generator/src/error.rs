use std::path::StripPrefixError;
use std::sync::mpsc::RecvError;
use zip::result::ZipError;
use std::fmt::Formatter;

pub fn error_with_message<T>(
    message: &str,
    error: impl std::fmt::Display,
) -> Result<T, ZagreusError> {
    Err(ZagreusError::from(format!("{}: {}", message, error)))
}

pub fn simple_error<T>(message: &str) -> Result<T, ZagreusError> {
    Err(ZagreusError::from(message.to_string()))
}

#[derive(Debug)]
pub struct ZagreusError {
    pub error_message: String,
}

impl ZagreusError {
    pub fn from(error_message: String) -> ZagreusError {
        ZagreusError { error_message }
    }
}

impl std::fmt::Display for ZagreusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error_message)
    }
}

impl From<std::io::Error> for ZagreusError {
    fn from(error: std::io::Error) -> Self {
        Self {
            error_message: "IO Error occurred: ".to_owned() + error.to_string().as_str(),
        }
    }
}

impl From<serde_json::error::Error> for ZagreusError {
    fn from(error: serde_json::error::Error) -> Self {
        Self {
            error_message: "JSON error occurred: ".to_owned() + error.to_string().as_str(),
        }
    }
}

impl From<serde_yaml::Error> for ZagreusError {
    fn from(error: serde_yaml::Error) -> Self {
        Self {
            error_message: "YAML error occurred: ".to_owned() + error.to_string().as_str(),
        }
    }
}

impl From<zip::result::ZipError> for ZagreusError {
    fn from(error: ZipError) -> Self {
        Self {
            error_message: format!("ZIP error occurred: {:?}.", error),
        }
    }
}

impl From<reqwest::Error> for ZagreusError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            error_message: format!("Reqwest error occurred: {}.", error),
        }
    }
}

impl From<notify::Error> for ZagreusError {
    fn from(error: notify::Error) -> Self {
        Self {
            error_message: format!("Notify error occurred: {}.", error),
        }
    }
}

impl From<&notify::Error> for ZagreusError {
    fn from(error: &notify::Error) -> Self {
        Self {
            error_message: format!("Notify error occurred: {}.", error),
        }
    }
}

impl From<RecvError> for ZagreusError {
    fn from(error: RecvError) -> Self {
        Self {
            error_message: format!("Recv error occurred: {}.", error),
        }
    }
}

impl From<StripPrefixError> for ZagreusError {
    fn from(error: StripPrefixError) -> Self {
        Self {
            error_message: format!("StripPrefixError error occurred: {}.", error),
        }
    }
}
