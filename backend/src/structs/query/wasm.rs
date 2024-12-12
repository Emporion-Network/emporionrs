use base64::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::Query;

pub struct QueryWasm<T: WasmQuery> {
    pub contract_address:String,
    pub query_data:T,
}

pub trait WasmQuery:Serialize {
    type Output:DeserializeOwned;
}

impl<T:WasmQuery> Query for QueryWasm<T> {
    type Output = T::Output;

    fn path(&self, rest:&str)->String {
        let query_data = serde_json::to_string(&self.query_data).unwrap();
        let r = BASE64_URL_SAFE.encode(query_data);
        println!("{}",r);
        format!("{}/cosmwasm/wasm/v1/contract/{}/smart/{}",rest, self.contract_address, r)
    }
}


#[derive(serde::Serialize, Default)]
struct Empty {}

#[derive(serde::Serialize, Default)]
pub struct Cw20TokenInfo {
    token_info:Empty,
}


#[derive(Deserialize, Debug)]
pub struct Cw20TokenInfoResp {
    pub name:String,
    pub symbol:String,
    pub decimals:String,
    pub total_supply:String
}

impl WasmQuery for Cw20TokenInfo {
    type Output = Cw20TokenInfoResp;
}
