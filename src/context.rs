use crate::config::ChatConfig;
use carapax::{types::Integer, Api};
use std::collections::HashMap;

pub struct Context {
    pub api: Api,
    pub chats: HashMap<Integer, ChatConfig>,
}
