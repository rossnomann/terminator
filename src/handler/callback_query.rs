use crate::{
    config::{Action, DEFAULT_NOTIFICATION_FORBIDDEN},
    context::{Context, Payload},
};
use carapax::{
    handler,
    methods::{AnswerCallbackQuery, DeleteMessage, KickChatMember, RestrictChatMember},
    types::CallbackQuery,
    ExecuteError,
};

#[handler]
pub async fn handle(context: &Context, query: CallbackQuery) -> Result<(), ExecuteError> {
    let answer = if let Ok(Some(data)) = query.parse_data::<Payload>() {
        let config = match context.chats.get(&data.chat_id) {
            Some(config) => config,
            None => return Ok(()),
        };
        if data.user_id == query.from.id {
            if let Some(message) = query.message {
                match context.api.execute(DeleteMessage::new(data.chat_id, message.id)).await {
                    Ok(_) => log::info!("Question #{} successfully deleted", message.id),
                    Err(err) => log::warn!("Failed to delete question: {}", err),
                };
            }
            if data.is_right {
                context
                    .api
                    .execute(RestrictChatMember::new(data.chat_id, data.user_id).allow_all())
                    .await?;
                config.notification_right()
            } else {
                if let Action::Kick = config.action_wrong() {
                    context
                        .api
                        .execute(KickChatMember::new(data.chat_id, data.user_id))
                        .await?;
                }
                config.notification_wrong()
            }
        } else {
            config.notification_forbidden()
        }
    } else {
        DEFAULT_NOTIFICATION_FORBIDDEN
    };
    context
        .api
        .execute(AnswerCallbackQuery::new(query.id).text(answer))
        .await?;
    Ok(())
}
