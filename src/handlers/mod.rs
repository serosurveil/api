//! Contains HTTP Handlers that directly receive and respond to requests to the server.
// mod analysis;
mod health_check;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
// pub use analysis::*;
pub use health_check::*;

pub type ApplicationResponse = Result<HttpResponse, ApplicationError>;

/// Error derived while handling an HTTP request
#[derive(thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Not Found")]
    NotFoundError(String),
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UnexpectedError(_e) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            Self::NotFoundError(_message) => HttpResponse::new(StatusCode::NOT_FOUND),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnexpectedError(_e) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError(_message) => StatusCode::NOT_FOUND,
        }
    }
}

pub fn json_response(data: impl serde::Serialize) -> ApplicationResponse {
    Ok(HttpResponse::Ok().json(data))
}

impl std::fmt::Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error::error_chain_fmt(self, f)
    }
}

impl From<sqlx::Error> for ApplicationError {
    fn from(e: sqlx::Error) -> Self {
        Self::UnexpectedError(anyhow::anyhow!(e))
    }
}

impl From<serde_json::Error> for ApplicationError {
    fn from(e: serde_json::Error) -> Self {
        Self::UnexpectedError(anyhow::anyhow!(e))
    }
}
