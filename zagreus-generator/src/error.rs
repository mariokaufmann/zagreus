use anyhow::anyhow;

pub fn error_with_message<T>(message: &str, error: impl std::fmt::Display) -> anyhow::Result<T> {
    Err(anyhow!(format!("{message}: {error}")))
}

pub fn simple_error<T>(message: &str) -> anyhow::Result<T> {
    Err(anyhow!(message.to_owned()))
}
