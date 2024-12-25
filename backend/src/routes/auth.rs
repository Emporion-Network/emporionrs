use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Extension, Json, Router,
};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::structs::{
    error::{map_err, new_err, Error},
    jwt::{jwt_middleware, Token},
    translate::TranslateReq,
};

#[axum::debug_handler]
async fn get_nonce(Extension(token): Extension<Token>) -> Result<Json<Token>, Error> {
    Ok(Json(token))
}

async fn translate_messages(
    Extension(_): Extension<Token>,
    Json(req): Json<TranslateReq>,
) -> Result<Json<Value>, Error> {
    let v = json!({
        "messages": [
            {
            "role": "system",
            "content": "You are a multilingual translation agent. You will be given JSON objects in the form {\"<source_language>\":\"<text>\", \"<target_language>\":\"\"}. Your task is to fill in the \"<target_language>\" field with the translated text."
            },
            {
            "role": "user",
            "content": "{\"fr\":\"Bonjour\", \"en\":\"\"}"
            },
            {
            "role": "assistant",
            "content": "{\"en\":\"Hello\"}"
            },
            {
            "role": "user",
            "content": "{\"es\":\"¿Cómo estás?\", \"de\":\"\"}"
            },
            {
            "role": "assistant",
            "content": "{\"de\":\"Wie geht es dir?\"}"
            },
            {
            "role": "user",
            "content": "{\"zh\":\"谢谢\", \"ar\":\"\"}"
            },
            {
            "role": "assistant",
            "content": "{\"ar\":\"شكراً\"}"
            },
            {
            "role": "user",
            "content": "{\"en\":\"Good morning\", \"ja\":\"\"}"
            },
            {
            "role": "assistant",
            "content": "{\"ja\":\"おはようございます\"}"
            },
            {
            "role": "user",
            "content": serde_json::to_string(&req.translations).unwrap()
            }
        ],
        "model": "gemma2-9b-it",
        "temperature": 1,
        "max_tokens": 1024,
        "top_p": 1,
        "stream": false,
        "response_format": {
          "type": "json_object"
        },
        "stop": null
    });
    let res: Value = reqwest::Client::new()
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header(
            "Authorization",
            format!("Bearer {}", std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")),
        )
        .json(&v)
        .send()
        .await
        .map_err(map_err("", StatusCode::BAD_REQUEST))?
        .json()
        .await
        .map_err(map_err("", StatusCode::BAD_REQUEST))?;
    let res = res
        .as_object()
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .get("choices")
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .as_array()
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .to_vec()
        .into_iter()
        .nth(0)
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?;
    let res = res
        .as_object()
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .get("message")
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .as_object()
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .get("content")
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?
        .as_str()
        .ok_or(new_err("Response not an object", StatusCode::BAD_REQUEST))?;
    let res = serde_json::from_str(res).unwrap();
    Ok(Json(res))
}

pub struct AuthParams {
    pub jwt_secret: String,
}

pub fn auth(params: &(impl Into<AuthParams> + Clone)) -> Router {
    let params: AuthParams = params.clone().into();
    Router::new()
        .route("/check", get(get_nonce))
        .route("/translate", post(translate_messages))
        .layer(from_fn_with_state(params.jwt_secret, jwt_middleware))
}
