use thiserror::Error;

use crate::resp::RespError;

pub type WalleResult<T> = Result<T, WalleError>;

/// Walle-core errors
#[derive(Error, Debug)]
pub enum WalleError {
    // event
    #[error("expect {0} found {1}")]
    DeclareNotMatch(&'static str, String),
    // action
    #[error("Action send error")]
    ActionSendError,
    // resp
    #[error("Action Response Timeout")]
    ResponseTimeout,
    #[error("RespMissmatch")]
    RespNotMatch, //todo
    #[error("{0:?}")]
    RespError(RespError),
    // server
    #[error("{0}")]
    IO(#[from] std::io::Error),
    // Running Time Error
    #[error("OneBot is already started")]
    AlreadyStarted,
    #[error("OneBot is not started")]
    NotStarted,

    // Extended
    #[error("ExtendedMap missed key: {0}")]
    MapMissedKey(String),
    #[error("Type mismatch expect {0}, got {1}")]
    ValueTypeNotMatch(String, String),
    #[error("Illegal base64")]
    IllegalBase64(String),

    // OBC
    #[error("Bot not exist")]
    BotNotExist,

    #[error("{0}")]
    Other(String),
}

impl serde::de::Error for WalleError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        WalleError::Other(format!("{}", msg))
    }
}

impl serde::ser::Error for WalleError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        WalleError::Other(format!("{}", msg))
    }
}

pub trait ResultExt<T, E> {
    fn ignore(self) -> Option<T>;
    fn log(self, target: &str) -> Option<T>;
}
impl<T, E: std::fmt::Debug> ResultExt<T, E> for Result<T, E> {
    #[inline(always)]
    fn ignore(self) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    #[inline(always)]
    fn log(self, target: &str) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                tracing::error!(target, "{:?}", e);
                None
            }
        }
    }
}
