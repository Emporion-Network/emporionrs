use axum::{body::to_bytes, response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub message:String,
    #[serde(skip)]
    pub code:StatusCode
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self)).into_response()
    }
}


pub async fn handle_error(e:Response) -> Response {
    if e.status().is_success() {
        return e
    }
    let mut resp =  Error {
        message:"Unknown error".to_string(),
        code:e.status()
    };
    let b = to_bytes(e.into_body(), 2048).await.unwrap();
    let p = serde_json::from_slice::<Error>(&b);
    if p.is_ok() {
        let p = p.unwrap();
        resp.message = p.message;
    }
    resp.into_response()
}


pub fn map_err<T>(message:&str, code:StatusCode) -> impl FnOnce(T) ->  Error{
    let message = message.to_owned();
    move |_|{
        Error {
            message:message.to_owned(),
            code,
        }
    }
}

pub fn new_err(message:&str, code:StatusCode) -> Error {
    Error {
        message:message.into(),
        code
    }
}
