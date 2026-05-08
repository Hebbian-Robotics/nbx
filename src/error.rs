use std::process::ExitCode;

use serde_json::{Value, json};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorCode {
    General,
    NotFound,
    AuthFailed,
    ValidationError,
    StreamError,
}

impl ErrorCode {
    pub fn exit_code(self) -> ExitCode {
        match self {
            Self::General | Self::StreamError => ExitCode::from(1),
            Self::NotFound => ExitCode::from(2),
            Self::AuthFailed => ExitCode::from(3),
            Self::ValidationError => ExitCode::from(4),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::NotFound => "not_found",
            Self::AuthFailed => "auth_failed",
            Self::ValidationError => "validation_error",
            Self::StreamError => "stream_error",
        }
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct NbxError {
    pub code: ErrorCode,
    pub message: String,
    pub detail: Value,
}

impl NbxError {
    pub fn new(code: ErrorCode, message: impl Into<String>, detail: Value) -> Self {
        Self {
            code,
            message: message.into(),
            detail,
        }
    }

    pub fn general(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::General, message, json!({}))
    }

    pub fn validation(message: impl Into<String>, detail: Value) -> Self {
        Self::new(ErrorCode::ValidationError, message, detail)
    }

    pub fn not_found(message: impl Into<String>, detail: Value) -> Self {
        Self::new(ErrorCode::NotFound, message, detail)
    }

    pub fn stream(message: impl Into<String>, detail: Value) -> Self {
        Self::new(ErrorCode::StreamError, message, detail)
    }

    pub fn exit_code(&self) -> ExitCode {
        self.code.exit_code()
    }
}

pub type NbxResult<T> = Result<T, NbxError>;
