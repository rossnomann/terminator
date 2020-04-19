use crate::config::ChatConfig;
use carapax::{
    session::{backend::fs::FilesystemBackend, SessionManager},
    types::Integer,
    Api,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Context {
    pub api: Api,
    pub chats: HashMap<Integer, ChatConfig>,
    pub session_manager: SessionManager<FilesystemBackend>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Payload {
    pub chat_id: Integer,
    pub user_id: Integer,
    pub is_right: bool,
}
