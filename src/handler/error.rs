use crate::config::RenderQuestionError;
use carapax::{session::SessionError, types::InlineKeyboardError, ExecuteError};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum HandlerError {
    Execute(ExecuteError),
    InlineKeyboard(InlineKeyboardError),
    LoadPermissions(SessionError),
    RenderQuestion(RenderQuestionError),
    SavePermissions(SessionError),
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
            LoadPermissions(err) => write!(out, "can not save chat member permissions: {}", err),
            RenderQuestion(err) => write!(out, "{}", err),
            SavePermissions(err) => write!(out, "can not save chat member permissions: {}", err),
        }
    }
}

impl Error for HandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::HandlerError::*;
        Some(match self {
            Execute(err) => err,
            InlineKeyboard(err) => err,
            LoadPermissions(err) => err,
            RenderQuestion(err) => err,
            SavePermissions(err) => err,
        })
    }
}
