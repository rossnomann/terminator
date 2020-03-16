use crate::config::raw::RawChatConfig;
use carapax::types::{Integer, MentionError, ParseMode, User};
use liquid::{value::liquid_value, Error as TemplateError, ParserBuilder as TemplateParserBuilder, Template};
use std::{collections::HashMap, error::Error, fmt, sync::Arc, time::Duration};

pub const DEFAULT_NOTIFICATION_RIGHT: &str = "Ok";
pub const DEFAULT_NOTIFICATION_WRONG: &str = "Wrong!";
pub const DEFAULT_NOTIFICATION_FORBIDDEN: &str = "You are not allowed to press this button!";

pub struct ChatConfig {
    question: Arc<Template>,
    buttons: Vec<ButtonConfig>,
    ask_timeout: Option<Duration>,
    response_timeout: Duration,
    notification_right: String,
    notification_wrong: String,
    notification_forbidden: String,
}

impl ChatConfig {
    pub fn render_question(&self, user: &User, parse_mode: ParseMode) -> Result<String, RenderQuestionError> {
        let user = user.get_mention(parse_mode).map_err(RenderQuestionError::Mention)?;
        let vars = liquid_value!({ "user": user })
            .into_object()
            .ok_or(RenderQuestionError::CreateVariables)?;
        let rendered = self
            .question
            .render(&vars)
            .map_err(RenderQuestionError::Render)?
            .trim()
            .to_string();
        Ok(rendered)
    }

    pub fn buttons(&self) -> &[ButtonConfig] {
        &self.buttons
    }

    pub fn ask_timeout(&self) -> Option<Duration> {
        self.ask_timeout
    }

    pub fn response_timeout(&self) -> Duration {
        self.response_timeout
    }

    pub fn notification_right(&self) -> &str {
        &self.notification_right
    }

    pub fn notification_wrong(&self) -> &str {
        &self.notification_wrong
    }

    pub fn notification_forbidden(&self) -> &str {
        &self.notification_forbidden
    }
}

pub struct ButtonConfig {
    label: String,
    is_right: bool,
}

impl ButtonConfig {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn is_right(&self) -> bool {
        self.is_right
    }
}

#[derive(Debug)]
pub enum RenderQuestionError {
    CreateVariables,
    Mention(MentionError),
    Render(TemplateError),
}

impl fmt::Display for RenderQuestionError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::RenderQuestionError::*;
        match self {
            CreateVariables => write!(out, "could not create variables for question template"),
            Mention(err) => write!(out, "can not get user mention string: {}", err),
            Render(err) => write!(out, "failed to render question: {}", err),
        }
    }
}

impl Error for RenderQuestionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::RenderQuestionError::*;
        match self {
            CreateVariables => None,
            Mention(err) => Some(err),
            Render(err) => Some(err),
        }
    }
}

pub(super) fn from_raw(raw: HashMap<Integer, RawChatConfig>) -> Result<HashMap<Integer, ChatConfig>, ChatConfigError> {
    let tpl_parser = TemplateParserBuilder::with_liquid()
        .build()
        .map_err(ChatConfigError::CreateTemplateParser)?;
    let mut result = HashMap::with_capacity(raw.len());
    for (chat_id, config) in raw {
        let question = Arc::new(
            tpl_parser
                .parse(&config.question)
                .map_err(ChatConfigError::ParseTemplate)?,
        );
        let buttons = config
            .buttons
            .into_iter()
            .map(|button| ButtonConfig {
                label: button.label,
                is_right: button.is_right,
            })
            .collect();
        let ask_timeout = config.ask_timeout.map(Duration::from_secs);
        let response_timeout = Duration::from_secs(config.response_timeout);
        let (notification_right, notification_wrong, notification_forbidden) = config
            .notification
            .map(|x| (x.right, x.wrong, x.forbidden))
            .unwrap_or_else(|| (None, None, None));
        let notification_right = notification_right.unwrap_or_else(|| String::from(DEFAULT_NOTIFICATION_RIGHT));
        let notification_wrong = notification_wrong.unwrap_or_else(|| String::from(DEFAULT_NOTIFICATION_WRONG));
        let notification_forbidden =
            notification_forbidden.unwrap_or_else(|| String::from(DEFAULT_NOTIFICATION_FORBIDDEN));
        result.insert(
            chat_id,
            ChatConfig {
                question,
                buttons,
                ask_timeout,
                response_timeout,
                notification_right,
                notification_wrong,
                notification_forbidden,
            },
        );
    }
    Ok(result)
}

#[derive(Debug)]
pub enum ChatConfigError {
    CreateTemplateParser(TemplateError),
    ParseTemplate(TemplateError),
}

impl fmt::Display for ChatConfigError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ChatConfigError::*;
        match self {
            CreateTemplateParser(err) => write!(out, "{}", err),
            ParseTemplate(err) => write!(out, "{}", err),
        }
    }
}

impl Error for ChatConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ChatConfigError::*;
        Some(match self {
            CreateTemplateParser(err) => err,
            ParseTemplate(err) => err,
        })
    }
}
