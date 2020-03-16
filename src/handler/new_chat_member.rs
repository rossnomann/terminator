use crate::{
    context::{Context, Payload},
    handler::error::HandlerError,
};
use carapax::{
    methods::{DeleteMessage, RestrictChatMember, SendMessage},
    types::{InlineKeyboardButton, Integer, ParseMode, User},
};

use tokio::{task, time::delay_for};

const PARSE_MODE: ParseMode = ParseMode::Html;

pub(super) async fn handle(context: &Context, chat_id: Integer, users: &[User]) -> Result<(), HandlerError> {
    let config = match context.chats.get(&chat_id) {
        Some(config) => config,
        None => {
            log::info!("Config not found for chat '{}'", chat_id);
            return Ok(());
        }
    };
    if let Some(timeout) = config.ask_timeout() {
        log::info!("Waiting for {} second(s) timeout before question", timeout.as_secs());
        delay_for(timeout).await;
    }
    for user in users {
        let user_id = user.id;
        context
            .api
            .execute(RestrictChatMember::new(chat_id, user_id).restrict_all())
            .await?;
        let question = config.render_question(&user, PARSE_MODE)?;
        let mut buttons = Vec::new();
        for button in config.buttons() {
            buttons.push(InlineKeyboardButton::with_callback_data_struct(
                button.label(),
                &Payload {
                    chat_id,
                    user_id,
                    is_right: button.is_right(),
                },
            )?)
        }
        let message = context
            .api
            .execute(
                SendMessage::new(chat_id, question)
                    .reply_markup(vec![buttons])
                    .parse_mode(PARSE_MODE),
            )
            .await?;
        let api = context.api.clone();
        let response_timeout = config.response_timeout();
        task::spawn(async move {
            log::info!(
                "Waiting for {} second(s) timeout before deleting question",
                response_timeout.as_secs()
            );
            delay_for(response_timeout).await;
            match api.execute(DeleteMessage::new(chat_id, message.id)).await {
                Ok(_) => log::info!("Question #{} successfully deleted", message.id),
                Err(err) => log::warn!("Failed to delete question: {}", err),
            }
        });
    }
    Ok(())
}
