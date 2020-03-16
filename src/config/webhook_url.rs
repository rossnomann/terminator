use std::net::{AddrParseError, SocketAddr};

const DEFAULT_PATH: &str = "/";

#[derive(Debug)]
pub struct WebhookUrl {
    pub address: SocketAddr,
    pub path: String,
}

impl WebhookUrl {
    pub(super) fn from_raw<A, P>(address: A, path: Option<P>) -> Result<Self, AddrParseError>
    where
        A: Into<String>,
        P: Into<String>,
    {
        let address = address.into().parse()?;
        let path = match path {
            Some(path) => path.into(),
            None => String::from(DEFAULT_PATH),
        };
        Ok(Self { address, path })
    }
}
