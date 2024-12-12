use std::{collections::HashMap, time::Duration};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::time::Instant;
use crate::structs::error::Error;

use super::error::map_err;



#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct  Price {
    usd:f64,
    usd_24h_change:f64,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[ts(type="Record<string,{usd:number, usd_24h_change:number}>")]
pub struct Prices {
    #[serde(flatten)]
    map:HashMap<String, Price>,
    #[serde(skip)]
    last_update:Option<Instant>,
    #[serde(skip)]
    url:String,
    #[serde(skip)]
    cache_duration:Duration
}

impl Prices {
    pub fn new<T:ToString>(ids:Vec<T>, cache_duration:Duration) -> Self{
        Self {
            last_update:None,
            map:HashMap::new(),
            url:format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&precision=3&include_24hr_change=true", ids.into_iter()
            .map(|item| item.to_string()) 
            .collect::<Vec<String>>().join(",")),
            cache_duration,
        }
    }

    pub fn is_empty(&self)->bool{
        self.map.is_empty()
    }

    pub async fn update(&mut self)-> Result<(), Error> {
        if self.last_update.is_some() && 
        Instant::now().duration_since(self.last_update.unwrap()) < self.cache_duration{
            return  Ok(());
        }
        self.map = reqwest::get(&self.url).await
        .map_err(map_err("could not fetch", StatusCode::SERVICE_UNAVAILABLE))?
        .json::<HashMap<String, Price>>()
        .await.map_err(map_err("could not fetch", StatusCode::SERVICE_UNAVAILABLE))?;
        self.last_update = Some(Instant::now());
        Ok(())
    }
}

#[tokio::test]
async fn test(){
    let mut x =  Prices::new(vec!["neutron-3", "bitcoin", "atom"], Duration::from_secs(10));
    x.update().await.unwrap();
}