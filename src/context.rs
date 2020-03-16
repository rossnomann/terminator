use crate::config::ChatConfig;
use carapax::{types::Integer, Api};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Context {
    pub api: Api,
    pub chats: HashMap<Integer, ChatConfig>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Payload {
    pub chat_id: Integer,
    pub user_id: Integer,
    pub is_right: bool,
}
