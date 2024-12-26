use axum::{
    extract::State, middleware::from_fn_with_state, routing::{get, post}, Extension, Json, Router
};
use serde_json::{json, Value};

use crate::structs::{
    error::Error,
    jwt::{jwt_middleware, Token},
    translate::TranslateReq,
};

#[axum::debug_handler]
async fn get_nonce(Extension(token): Extension<Token>) -> Result<Json<Token>, Error> {
    Ok(Json(token))
}

async fn translate_messages(
    Extension(_): Extension<Token>,
    State(auth_params):State<AuthParams>,
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
        "stop": null
    });
    let res: Value = reqwest::Client::new()
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header(
            "Authorization",
            format!("Bearer {}", auth_params.openai_api_key),
        )
        .json(&v)
        .send()
        .await?
        .json()
        .await?;
    let res:Option<_> = (||{
        let res = res.as_object()?
        .get("choices")?
        .as_array()?
        .to_vec()
        .into_iter()
        .nth(0)?;
        let res = res.as_object()?
        .get("message")?
        .as_object()?
        .get("content")?
        .as_str()?
        .to_string();
        Some(res)
    })();
    let res = res.ok_or(Error::Default)?;
    
    let res = serde_json::from_str(&res).unwrap();
    Ok(Json(res))
}

#[derive(Clone)]
pub struct AuthParams {
    pub jwt_secret: String,
    pub openai_api_key:String,
}

pub fn auth(params: &(impl Into<AuthParams> + Clone)) -> Router {
    let params: AuthParams = params.clone().into();
    Router::new()
        .route("/check", get(get_nonce))
        .route("/translate", post(translate_messages))
        .layer(from_fn_with_state(params.clone(), jwt_middleware))
        .with_state(params)
}
