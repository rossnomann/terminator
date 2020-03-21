use crate::{
    config::Action,
    context::{Context, Payload},
    handler::error::HandlerError,
};
use carapax::{
    methods::{DeleteMessage, KickChatMember, RestrictChatMember, SendMessage},
    types::{InlineKeyboardButton, Integer, ParseMode, User},
    Api,
};
use std::time::Duration;

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
        let timeout_handler = TimeoutHandler {
            api: context.api.clone(),
            timeout: config.response_timeout(),
            chat_id,
            message_id: message.id,
            user_id,
            action: config.action_timeout(),
        };
        task::spawn(timeout_handler.run());
    }
    Ok(())
}

struct TimeoutHandler {
    api: Api,
    timeout: Duration,
    chat_id: Integer,
    message_id: Integer,
    user_id: Integer,
    action: Action,
}

impl TimeoutHandler {
    async fn run(self) {
        log::info!(
            "Waiting for {} second(s) timeout before deleting question",
            self.timeout.as_secs()
        );
        delay_for(self.timeout).await;
        match self
            .api
            .execute(DeleteMessage::new(self.chat_id, self.message_id))
            .await
        {
            Ok(_) => {
                // User not respond to question
                log::info!("Question #{} successfully deleted", self.message_id);
                if let Action::Kick = self.action {
                    match self.api.execute(KickChatMember::new(self.chat_id, self.user_id)).await {
                        Ok(_) => log::info!("Chat member kicked"),
                        Err(err) => log::warn!(
                            "Failed to kick chat member (chat_id={}, user_id={}): {}",
                            self.chat_id,
                            self.user_id,
                            err
                        ),
                    }
                }
            }
            Err(err) => {
                // Possibly user respond to question
                log::info!("Failed to delete question: {}", err)
            }
        }
    }
}
