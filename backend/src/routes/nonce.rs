use axum::{
    extract::{Json, State}, http::StatusCode, routing::{post, Router}
};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::structs::{
    error::{map_err, Error}, jwt::{Token, TokenResp}, nonce::{NonceReq, NonceRes, NonceToAddr}, sign_doc::Signature
};

#[axum::debug_handler]
async fn get_nonce(
    State(params): State<Arc<Mutex<NonceState>>>,
    Json(req): Json<NonceReq>,
) -> Result<Json<NonceRes>, Error> {
    bech32::decode(&req.address).map_err(map_err("Address not valid", StatusCode::BAD_REQUEST))?;
    let mut params = params.lock().await;
    params.nonces.add(&req.address).and_then(|t| Ok(Json(t)))
}

async fn check_nonce(
    State(params): State<Arc<Mutex<NonceState>>>,
    Json(singature): Json<Signature>,
) -> Result<Json<TokenResp>, Error> {
    let addr = singature.verify()?;
    let mut params = params.lock().await;
    params.nonces.check(&singature.nonce, &addr)?;
    Token::new(addr, params.jwt_lifetime).into_resp(&params.jwt_secret)
}

pub struct NonceParams {
    pub jwt_lifetime:Duration,
    pub jwt_secret:String,
    pub nonce_lifetime:Duration
}

struct NonceState {
    jwt_lifetime:Duration,
    jwt_secret:String,
    nonces:NonceToAddr,
}

pub fn nonce(params:&(impl Into<NonceParams> + Clone)) -> Router {
    let params:NonceParams = (params.clone()).into();
    let state = Arc::new(Mutex::new(NonceState {
        nonces:NonceToAddr::new(params.nonce_lifetime),
        jwt_lifetime: params.jwt_lifetime,
        jwt_secret: params.jwt_secret,
    }));
    Router::new()
        .route("/nonce", post(get_nonce))
        .route("/check-nonce", post(check_nonce))
        .with_state(state)
}
