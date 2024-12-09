use axum::{middleware::from_fn_with_state, routing::get, Extension, Json, Router};

use crate::structs::{
    jwt::{jwt_middleware, Token},
    error::Error
};


#[axum::debug_handler]
async fn get_nonce(
    Extension(token):Extension<Token>
)->Result<Json<Token>, Error>{
    Ok(Json(token))
}

pub struct AuthParams {
    pub jwt_secret:String
}

pub fn auth(params:&(impl Into<AuthParams> + Clone)) -> Router {
    let params:AuthParams = params.clone().into();
    Router::new()
        .route("/authed", get(get_nonce))
        .layer(from_fn_with_state(params.jwt_secret, jwt_middleware))
}