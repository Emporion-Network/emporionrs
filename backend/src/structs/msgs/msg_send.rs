use serde::{Deserialize, Serialize};

use crate::exact_string;

exact_string!(exact_type, "cosmos.bank.v1beta1.MsgSend");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageSend {
    #[serde(rename = "@type")]
    #[serde(deserialize_with="exact_type")]
    pub r#type: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Coin>,
}
