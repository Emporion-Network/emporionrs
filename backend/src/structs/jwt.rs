use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::error::Error;

use super::error::{map_err, new_err};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Token {
    pub addr: String,
    pub exp: usize,
}

impl Token {
    pub fn new(addr: String, lifetime: Duration) -> Self {
        let r = SystemTime::now() + lifetime;
        let exp = r.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Self { addr, exp }
    }

    pub fn into_resp(self, key: &str) -> Result<Json<TokenResp>, Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &EncodingKey::from_secret(key.as_ref()),
        )
        .map_err(map_err(
            "Could not create token",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;
        Ok(Json(TokenResp { token }))
    }
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TokenResp {
    pub token: String,
}

impl TokenResp {
    pub fn try_into_token(&self, jwt_secret:&str) -> Result<Token, Error>{
        jsonwebtoken::decode::<Token>(
            &self.token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ).map(|e| e.claims)
        .map_err(map_err("Token not valid", StatusCode::UNAUTHORIZED))
    }
}



pub async fn jwt_middleware(
    State(jwt_secret): State<String>,
    mut req: Request,
    next: Next,
) -> Result<Response, Error> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(new_err(
            "Missing Authorization header",
            StatusCode::UNAUTHORIZED,
        ))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(new_err("Invalid token format", StatusCode::UNAUTHORIZED));
    }
    let token = auth_header["Bearer ".len()..].to_string();
    let token = jsonwebtoken::decode::<Token>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(map_err("Invalid token", StatusCode::UNAUTHORIZED))?
    .claims;
    req.extensions_mut().insert(token);

    Ok(next.run(req).await)
}
