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
    log::info!(
        "Got a message from chat {} (chat username: {:?})",
        message.get_chat_id(),
        message.get_chat_username()
    );
    if let MessageData::NewChatMembers(ref users) = message.data {
        let chat_id = message.get_chat_id();
        new_chat_member::handle(context, chat_id, message.id, users).await?;
    }
    Ok(())
}
