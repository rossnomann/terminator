use crate::{
    context::Context,
    handler::{error::HandlerError, new_chat_member},
};
use carapax::{
    handler,
    types::{Message, MessageData},
};

#[handler]
pub async fn handle(context: &Context, message: Message) -> Result<(), HandlerError> {
    if let MessageData::NewChatMembers(ref users) = message.data {
        let chat_id = message.get_chat_id();
        new_chat_member::handle(context, chat_id, users).await?;
    }
    Ok(())
}
