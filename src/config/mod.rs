use carapax::{Config as ApiConfig, ParseProxyError};
use std::{collections::HashMap, error::Error, fmt, net::AddrParseError, path::Path};

mod chat;
mod raw;
mod webhook_url;

use self::{
    chat::ChatConfigError,
    raw::{RawConfig, RawConfigError},
};

pub use self::{
    chat::{
        ButtonConfig, ChatConfig, RenderQuestionError, DEFAULT_NOTIFICATION_FORBIDDEN, DEFAULT_NOTIFICATION_RIGHT,
        DEFAULT_NOTIFICATION_WRONG,
    },
    webhook_url::WebhookUrl,
};

pub struct Config {
    pub api: ApiConfig,
    pub webhook_url: Option<WebhookUrl>,
    pub chats: HashMap<i64, ChatConfig>,
}

impl Config {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let raw = RawConfig::from_file(path).await?;

        let mut api = ApiConfig::new(raw.token.clone());
        if let Some(ref proxy) = raw.proxy {
            api = api.proxy(proxy.clone())?;
        }

        let webhook_url = match raw.webhook_address {
            Some(addr) => Some(WebhookUrl::from_raw(addr, raw.webhook_path).map_err(ConfigError::WebhookAddress)?),
            None => None,
        };

        let chats = chat::from_raw(raw.chats)?;

        Ok(Config {
            api,
            webhook_url,
            chats,
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Chat(ChatConfigError),
    ParseProxy(ParseProxyError),
    Raw(RawConfigError),
    WebhookAddress(AddrParseError),
}

impl From<ChatConfigError> for ConfigError {
    fn from(err: ChatConfigError) -> Self {
        ConfigError::Chat(err)
    }
}

impl From<ParseProxyError> for ConfigError {
    fn from(err: ParseProxyError) -> Self {
        ConfigError::ParseProxy(err)
    }
}

impl From<RawConfigError> for ConfigError {
    fn from(err: RawConfigError) -> Self {
        ConfigError::Raw(err)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ConfigError::*;
        Some(match self {
            Chat(err) => err,
            ParseProxy(err) => err,
            Raw(err) => err,
            WebhookAddress(err) => err,
        })
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ConfigError::*;
        match self {
            Chat(err) => write!(out, "{}", err),
            ParseProxy(err) => write!(out, "bad proxy address: {}", err),
            Raw(err) => write!(out, "{}", err),
            WebhookAddress(err) => write!(out, "bad webhook address: {}", err),
        }
    }
}
