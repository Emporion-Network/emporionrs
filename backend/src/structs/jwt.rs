use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{routes::auth::AuthParams, structs::error::Error};

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

    pub fn into_resp(self, key: &str) -> Result<Json<TokenRes>, Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &EncodingKey::from_secret(key.as_ref()),
        )?;
        Ok(Json(TokenRes { token }))
    }
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TokenRes {
    pub token: String,
}

impl TokenRes {
    pub fn try_into_token(&self, jwt_secret:&str) -> Result<Token, Error>{
        Ok(jsonwebtoken::decode::<Token>(
            &self.token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ).map(|e| e.claims)?)
    }
}



pub async fn jwt_middleware(
    State(auth_params): State<AuthParams>,
    mut req: Request,
    next: Next,
) -> Result<Response, Error> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(Error::unauthorized(
            "Missing Authorization header",
        ))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(Error::unauthorized("Invalid token format"));
    }
    let token = auth_header["Bearer ".len()..].to_string();
    let token = jsonwebtoken::decode::<Token>(
        &token,
        &DecodingKey::from_secret(auth_params.jwt_secret.as_ref()),
        &Validation::default(),
    )?
    .claims;
    req.extensions_mut().insert(token);

    Ok(next.run(req).await)
}
