use crate::{
    config::{Config, ConfigError, WebhookUrl},
    context::Context,
    handler::{on_callback_query, on_message},
};
use carapax::{
    longpoll::LongPoll,
    session::{backend::fs::FilesystemBackend, SessionCollector, SessionManager},
    webhook, Api, ApiError, Dispatcher,
};
use hyper::Error as HyperError;
use std::{env, fmt, io::Error as IoError, time::Duration};
use tempfile::tempdir;

const SESSION_GC_PERIOD: Duration = Duration::from_secs(86400);
const SESSION_LIFETIME: Duration = Duration::from_secs(43200);

pub async fn run() -> Result<(), Error> {
    env_logger::init();
    let Config {
        api: api_config,
        webhook_url,
        chats,
    } = match env::args().nth(1) {
        Some(path) => Config::from_file(path).await?,
        None => return Err(Error::ConfigPathMissing),
    };
    let session_path = tempdir().map_err(Error::CreateSessionDirectory)?.into_path();
    let session_backend = FilesystemBackend::new(session_path);
    let session_manager = SessionManager::new(session_backend.clone());
    let api = Api::new(api_config)?;
    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        chats,
        session_manager,
    });
    dispatcher.add_handler(on_message);
    dispatcher.add_handler(on_callback_query);

    tokio::spawn(async move {
        SessionCollector::new(session_backend, SESSION_GC_PERIOD, SESSION_LIFETIME)
            .run()
            .await;
    });

    match webhook_url {
        Some(WebhookUrl { address, path }) => {
            log::info!("Starting receiving updates via webhook: {}{}", address, path);
            webhook::run_server(address, path, dispatcher)
                .await
                .map_err(Error::Webhook)?;
        }
        None => {
            log::info!("Starting receiving updates via long polling");
            LongPoll::new(api, dispatcher).run().await;
        }
    };
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Config(ConfigError),
    ConfigPathMissing,
    CreateSessionDirectory(IoError),
    Webhook(HyperError),
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::Api(err)
    }
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Error::Config(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            Api(err) => write!(out, "{}", err),
            Config(err) => write!(out, "{}", err),
            ConfigPathMissing => write!(out, "You need to provide a path to config"),
            CreateSessionDirectory(err) => write!(out, "Failed to create session directory: {}", err),
            Webhook(err) => write!(out, "Webhook error: {}", err),
        }
    }
}
