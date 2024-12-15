use crate::exact_string;

use super::msg_send::Coin;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

exact_string!(exact_type, "ibc.applications.transfer.v1.MsgTransfer");

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
pub struct IbcTransfer {
    #[serde(rename = "@type")]
    #[serde(deserialize_with="exact_type")]
    pub r#type: String,
    pub receiver: String,
    pub sender: String,
    pub source_channel: String,
    pub source_port: String,
    pub token: Coin,
}
