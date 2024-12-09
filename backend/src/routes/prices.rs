use axum::{extract::State, routing::get, Json, Router};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::structs::error::Error;
use crate::structs::prices::Prices;

#[axum::debug_handler]
async fn handler(State(prices): State<Arc<Mutex<Prices>>>) -> Result<Json<Prices>, Error> {
    let mut x = prices.lock().await;
    let updated = x.update().await;
    if updated.is_err() && x.is_empty() {
        updated?
    }
    Ok(Json(x.clone()))
}

pub struct PricesParams {
    pub coin_ids: Vec<String>,
    pub cache_duration: Duration,
}

pub fn get_prices(params: &(impl Into<PricesParams> + Clone)) -> Router {
    let params: PricesParams = params.clone().into();
    let prices = Arc::new(Mutex::new(Prices::new(
        params.coin_ids,
        params.cache_duration,
    )));
    Router::new()
        .route("/prices", get(handler))
        .with_state(prices)
}
