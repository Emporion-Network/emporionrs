use serde::{Deserialize, Serialize};

use crate::exact_string;

exact_string!(exact_type, "ibc.core.channel.v1.MsgRecvPacket");


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedData {
    pub amount:String,
    pub denom:String,
    pub receiver:String,
    pub sender:String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncodedData {
    pub data:String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Data {
    DecodedData(DecodedData),
    EncodedData(EncodedData)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paket {
    #[serde(flatten)]
    pub data:Data,
    pub destination_channel:String,
    pub destination_port:String,
    pub source_channel:String,
    pub source_port:String,
    pub sequence:u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IbcRecieve {
    #[serde(rename = "@type")]
    #[serde(deserialize_with="exact_type")]
    pub r#type:String,
    pub packet:Paket,
}