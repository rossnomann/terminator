use serde::Deserialize;
use serde_yaml::Error as YamlError;
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    io::Error as IoError,
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Deserialize)]
pub(super) struct RawConfig {
    pub(super) token: String,
    pub(super) proxy: Option<String>,
    pub(super) webhook_address: Option<String>,
    pub(super) webhook_path: Option<String>,
    pub(super) chats: HashMap<i64, RawChatConfig>,
}

#[derive(Deserialize)]
pub(super) struct RawChatConfig {
    pub(super) question: String,
    pub(super) buttons: Vec<RawButtonConfig>,
    pub(super) ask_timeout: Option<u64>,
    pub(super) response_timeout: u64,
    pub(super) notification: Option<RawNotificationConfig>,
}

#[derive(Deserialize)]
pub(super) struct RawButtonConfig {
    pub(super) label: String,
    pub(super) is_right: bool,
}

#[derive(Deserialize)]
pub(super) struct RawNotificationConfig {
    pub(super) right: Option<String>,
    pub(super) wrong: Option<String>,
    pub(super) forbidden: Option<String>,
}

impl RawConfig {
    pub(super) async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, RawConfigError> {
        let path = path.as_ref();
        let data = fs::read(path)
            .await
            .map_err(|err| RawConfigError::ReadFile(path.to_owned(), err))?;
        let raw = serde_yaml::from_slice(&data)?;
        Ok(raw)
    }
}

#[derive(Debug)]
pub enum RawConfigError {
    ReadFile(PathBuf, IoError),
    ParseYaml(YamlError),
}

impl From<YamlError> for RawConfigError {
    fn from(err: YamlError) -> Self {
        RawConfigError::ParseYaml(err)
    }
}

impl fmt::Display for RawConfigError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::RawConfigError::*;
        match self {
            ReadFile(path, err) => write!(out, "failed to read a file '{}': {}", path.display(), err),
            ParseYaml(err) => write!(out, "failed to parse YAML: {}", err),
        }
    }
}

impl Error for RawConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::RawConfigError::*;
        Some(match self {
            ReadFile(_, err) => err,
            ParseYaml(err) => err,
        })
    }
}
