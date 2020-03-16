use crate::config::RenderQuestionError;
use carapax::{types::InlineKeyboardError, ExecuteError};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum HandlerError {
    Execute(ExecuteError),
    InlineKeyboard(InlineKeyboardError),
    RenderQuestion(RenderQuestionError),
}

impl From<ExecuteError> for HandlerError {
    fn from(err: ExecuteError) -> Self {
        HandlerError::Execute(err)
    }
}

impl From<InlineKeyboardError> for HandlerError {
    fn from(err: InlineKeyboardError) -> Self {
        HandlerError::InlineKeyboard(err)
    }
}

impl From<RenderQuestionError> for HandlerError {
    fn from(err: RenderQuestionError) -> Self {
        HandlerError::RenderQuestion(err)
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::HandlerError::*;
        match self {
            Execute(err) => write!(out, "failed to execute method: {}", err),
            InlineKeyboard(err) => write!(out, "can not build inline keyboard: {}", err),
            RenderQuestion(err) => write!(out, "{}", err),
        }
    }
}

impl Error for HandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::HandlerError::*;
        Some(match self {
            Execute(err) => err,
            InlineKeyboard(err) => err,
            RenderQuestion(err) => err,
        })
    }
}
