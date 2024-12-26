use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;
use ts_rs::TS;

#[derive(Error, Debug, TS)]
#[ts(export)]
#[ts(type = "{error:string}")]
pub enum Error {
    #[error("Internal server error")]
    Default,
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    Bech(#[from] bech32::DecodeError),
    #[error("{0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    TooManyRequests(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("Service unavailabe")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    DeserializeJson(#[from] serde_json::Error),
    #[error("{0}")]
    DeserializeBase64(#[from] base64::DecodeError),
    #[error("{0}")]
    Signature(#[from] secp256k1::Error),
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error::Custom(message.to_string())
    }
    pub fn unauthorized(message: &str) -> Self {
        Error::Unauthorized(message.to_string())
    }
    pub fn too_many_requests(message: &str) -> Self {
        Error::TooManyRequests(message.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let code = match &self {
            Error::Default => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Custom(_) => StatusCode::BAD_REQUEST,
            Error::Bech(_) => StatusCode::BAD_REQUEST,
            Error::JWT(_) => StatusCode::BAD_REQUEST,
            Error::DeserializeJson(_) => StatusCode::BAD_REQUEST,
            Error::DeserializeBase64(_) => StatusCode::BAD_REQUEST,
            Error::Signature(_) => StatusCode::BAD_REQUEST,
            Error::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Reqwest(_) => StatusCode::SERVICE_UNAVAILABLE,
        };
        (
            code,
            Json(json!({
                "error":format!("{}", self),
            })),
        )
            .into_response()
    }
}
