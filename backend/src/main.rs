use std::time::Duration;

use axum::Router;
use routes::{auth::AuthParams, blockchain::handle_blockchain_events, chat::handle_chat_messages, nonce::NonceParams, prices::PricesParams, ws::WsParams};
use structs::notification::Notifier;
use tower_http::cors::CorsLayer;

mod routes;
pub mod structs;
pub mod utils;

#[derive(Clone)]
struct Params {
    nonce_lifetime: Duration,
    jwt_lifetime: Duration,
    jwt_secret: String,
    coin_ids: Vec<String>,
    cache_duration: Duration,
    notifier:Notifier
}

impl Into<NonceParams> for Params {
    fn into(self) -> NonceParams {
        NonceParams {
            jwt_lifetime: self.jwt_lifetime,
            jwt_secret: self.jwt_secret,
            nonce_lifetime: self.nonce_lifetime,
        }
    }
}

impl Into<AuthParams> for Params {
    fn into(self) -> AuthParams {
        AuthParams {
            jwt_secret: self.jwt_secret,
        }
    }
}

impl Into<PricesParams> for Params {
    fn into(self) -> PricesParams {
        PricesParams {
            coin_ids: self.coin_ids,
            cache_duration: self.cache_duration,
        }
    }
}

impl Into<WsParams> for Params {
    fn into(self) -> WsParams {
        WsParams {
            notifier:self.notifier,
            jwt_secret: self.jwt_secret,
        }
    }
}

#[tokio::main]
async fn main() {

    let params = Params {
        nonce_lifetime: Duration::from_secs(10),
        jwt_lifetime: Duration::from_secs(10),
        jwt_secret: "".to_string(),
        coin_ids: vec!["neutron-3", "bitcoin", "atom"]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>(),
        cache_duration: Duration::from_secs(60),
        notifier:Notifier::new(),
    };

    handle_chat_messages(&params.notifier);
    // handle_blockchain_events(&params.notifier);

    
    let app = Router::new()
        .merge(routes::prices::get_prices(&params))
        .merge(routes::nonce::nonce(&params))
        .merge(routes::auth::auth(&params))
        .merge(routes::ws::ws(&params))
        .layer(CorsLayer::very_permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
