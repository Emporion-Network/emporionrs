use std::{fs, sync::LazyLock};

use crate::structs::notification::Entity;
use base64::{prelude::BASE64_STANDARD, Engine};
use futures::{SinkExt, StreamExt};
use prost_reflect::{DescriptorPool, DynamicMessage, FieldDescriptor, Kind};
use protobuf::{descriptor::FileDescriptorSet, SpecialFields};
use protobuf_parse::Parser;
use secp256k1::hashes::hex::DisplayHex;
use secp256k1::hashes::hex::FromHex;
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::query::Query;
use super::{
    msgs::{ibc_recieve::IbcRecieve, ibc_transfer::IbcTransfer, msg_send::MessageSend},
    notification::{Notification, Notifier},
};
use crate::structs::notification::Data::BlockchainNotification as NotificationData;

pub struct Blockchain {
    websoket: String,
    rest: String,
}

pub static DESCRIPTOR_POOL: LazyLock<DescriptorPool> = LazyLock::new(init_descriptor_pool);

fn get_files(path: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut entries = fs::read_dir(path).unwrap();
    while let Some(Ok(entry)) = entries.next() {
        let ftype = entry.file_type().unwrap();
        if ftype.is_dir() {
            let mut r = get_files(entry.path().to_str().unwrap());
            res.append(&mut r);
            continue;
        }
        if entry.file_name().to_str().unwrap().ends_with(".proto") {
            res.push(entry.path().to_str().unwrap().to_string());
        }
    }
    res
}

fn init_descriptor_pool() -> DescriptorPool {
    use protobuf::Message;
    let mut p = Parser::new();
    let f = get_files("proto");
    let r = p.include("proto").inputs(f).parse_and_typecheck().unwrap();
    let set = FileDescriptorSet {
        file: r.file_descriptors,
        special_fields: SpecialFields::new(),
    };
    let d = set.write_to_bytes().unwrap();
    let r = DescriptorPool::decode(d.as_slice()).unwrap();
    r
}

fn to_json(fd: FieldDescriptor, v: &prost_reflect::Value) -> Value {
    match fd.kind() {
        Kind::Double => Value::Number(Number::from_f64(v.as_f64().unwrap_or_default()).unwrap()),
        Kind::Float => {
            Value::Number(Number::from_f64(v.as_f32().unwrap_or_default().into()).unwrap())
        }
        Kind::Int32 => {
            Value::Number(Number::from_i128(v.as_i32().unwrap_or_default().into()).unwrap())
        }
        Kind::Int64 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Uint32 => {
            Value::Number(Number::from_i128(v.as_u32().unwrap_or_default().into()).unwrap())
        }
        Kind::Uint64 => {
            Value::Number(Number::from_i128(v.as_u64().unwrap_or_default().into()).unwrap())
        }
        Kind::Sint32 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Sint64 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Fixed32 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Fixed64 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Sfixed32 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Sfixed64 => {
            Value::Number(Number::from_i128(v.as_i64().unwrap_or_default().into()).unwrap())
        }
        Kind::Bool => Value::Bool(v.as_bool().unwrap_or_default()),
        Kind::String => Value::String(v.as_str().unwrap_or_default().to_string()),
        Kind::Bytes => Value::String(v.as_bytes().unwrap().to_lower_hex_string()),
        Kind::Enum(enum_descriptor) => {
            let v = v.as_enum_number().unwrap();
            Value::String(
                enum_descriptor
                    .get_value(v)
                    .unwrap()
                    .full_name()
                    .to_string(),
            )
        }
        Kind::Message(_) => unreachable!(),
    }
}

fn parse_message(msg: DynamicMessage) -> Value {
    let mut ret = Map::new();
    let msg = if let Some((type_url, decoder)) = msg
        .get_field_by_name("type_url")
        .and_then(|v| v.as_str().map(|v| v[1..].to_string()))
        .and_then(|t| DESCRIPTOR_POOL.get_message_by_name(&t).map(|d| (t, d)))
    {
        let value = msg
            .get_field_by_name("value")
            .unwrap()
            .as_bytes()
            .unwrap()
            .to_vec();
        let msg = DynamicMessage::decode(decoder, value.as_slice()).unwrap();
        ret.insert("@type".to_string(), Value::String(type_url));
        msg
    } else {
        msg
    };

    msg.fields().for_each(|(f, v)| {
        if let Some(_) = v.as_message() {
            ret.insert(
                f.json_name().to_string(),
                parse_message(v.as_message().unwrap().clone()),
            );
        } else if let Some(_) = v
            .as_list()
            .and_then(|l| l.iter().all(|v| v.as_message().is_some()).then(|| true))
        {
            let r = v
                .as_list()
                .unwrap()
                .iter()
                .map(|v| parse_message(v.as_message().unwrap().clone()))
                .collect::<Vec<_>>();
            ret.insert(f.json_name().to_string(), Value::Array(r));
        } else {
            ret.insert(f.json_name().to_string(), to_json(f, v));
        }
    });
    return Value::Object(ret);
}

fn parse_messages(tx: DynamicMessage) -> Vec<Value> {
    let body = tx.get_field_by_name("body").unwrap();
    let body = body
        .as_message()
        .unwrap()
        .get_field_by_name("messages")
        .unwrap();
    let messages = body
        .as_list()
        .unwrap()
        .into_iter()
        .filter_map(|v| {
            let m = v.as_message()?;
            let msg = parse_message(m.clone());
            Some(msg)
        })
        .collect::<Vec<_>>();
    messages
}

fn parse_resp(block_res: Value) -> Option<Vec<TxMessage>> {
    let events = block_res.get("result")?.get("events")?;
    let tx_result = block_res
        .get("result")?
        .get("data")?
        .get("value")?
        .get("TxResult")?;
    let height = tx_result.get("height")?;
    let height = height.as_str().to_owned()?;
    if tx_result.get("result")?.get("data").is_none() {
        return None;
    }
    let hash = events
        .as_object()?
        .get("tx.hash")?
        .as_array()?
        .get(0)?
        .as_str()?;
    let tx = tx_result.get("tx")?;
    let tx_decoder = DESCRIPTOR_POOL
        .get_message_by_name("cosmos.tx.v1beta1.Tx")
        .unwrap();
    let tx = BASE64_STANDARD.decode(tx.as_str().unwrap()).ok()?;
    let tx = DynamicMessage::decode(tx_decoder, tx.as_slice()).unwrap();
    let res = parse_messages(tx)
        .into_iter()
        .map(|m| TxMessage {
            hash: hash.to_owned(),
            height: height.to_owned(),
            message: m,
        })
        .collect();
    return Some(res);
    // None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TxMessage {
    hash: String,
    height: String,
    message: Value,
}

impl TryInto<BlockchainNotification> for TxMessage {
    type Error = ();
    fn try_into(self) -> Result<BlockchainNotification, ()> {
        let data: Data = serde_json::from_value(self.message).map_err(|_| ())?;
        Ok(BlockchainNotification {
            hash: self.hash,
            height: self.height,
            data,
        })
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Data {
    MessageSend(MessageSend),
    IbcTransfer(IbcTransfer),
    IbcRecieve(IbcRecieve),
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct BlockchainNotification {
    hash: String,
    height: String,
    data: Data,
}


impl Into<Vec<Notification>> for BlockchainNotification {
    fn into(self) -> Vec<Notification> {
        match &self.data {
            Data::MessageSend(message_send) => {
                vec![
                    Notification {
                        r#for: Entity::User(message_send.from_address.clone()),
                        from: Entity::Blockchain {},
                        data: NotificationData(self.clone()),
                    },
                    Notification {
                        r#for: Entity::User(message_send.to_address.clone()),
                        from: Entity::Blockchain {},
                        data: NotificationData(self),
                    },
                ]
            }
            Data::IbcTransfer(ibc_transfer) => {
                vec![
                    Notification {
                        r#for: Entity::User(ibc_transfer.sender.clone()),
                        from: Entity::Blockchain {},
                        data: NotificationData(self.clone()),
                    },
                    Notification {
                        r#for: Entity::User(ibc_transfer.sender.clone()),
                        from: Entity::Blockchain {},
                        data: NotificationData(self),
                    },
                ]
            }
            Data::IbcRecieve(ibc_recieve) => {
                let decoded_data = match &ibc_recieve.packet.data {
                    crate::structs::msgs::ibc_recieve::Data::DecodedData(decoded_data) => {
                        decoded_data.clone()
                    }
                    crate::structs::msgs::ibc_recieve::Data::EncodedData(hex) => {
                        serde_json::from_slice(&<Vec<u8>>::from_hex(&hex.data).unwrap()).unwrap()
                    }
                };
                let sender = decoded_data.sender.clone();
                let receiver = decoded_data.receiver.clone();
                let mut data = ibc_recieve.clone();
                data.packet.data =
                    crate::structs::msgs::ibc_recieve::Data::DecodedData(decoded_data);
                vec![
                    Notification {
                        r#for: Entity::User(sender),
                        from: Entity::Blockchain {},
                        data: NotificationData(BlockchainNotification {
                            data: Data::IbcRecieve(data.clone()),
                            ..self.clone()
                        }),
                    },
                    Notification {
                        r#for: Entity::User(receiver),
                        from: Entity::Blockchain {},
                        data: NotificationData(BlockchainNotification {
                            data: Data::IbcRecieve(data),
                            ..self.clone()
                        }),
                    },
                ]
            }
        }
    }
}

impl Blockchain {
    fn listen(self, notifier: Notifier) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let (mut ws_stream, _) = connect_async(self.websoket).await.expect("Failed to connect");
            let msg = Message::text("{\"jsonrpc\": \"2.0\", \"method\": \"subscribe\", \"params\": [\"tm.event='Tx'\"], \"id\": 1 }");
            let _ = ws_stream.send(msg).await;

            loop {
                let m = match ws_stream.next().await {
                    Some(Ok(m)) => m.into_text(),
                    Some(Err(_)) => {
                        break;
                    }
                    _ => continue,
                };
                if m.is_err() {
                    continue;
                }
                let m = m.unwrap();
                let m = serde_json::from_str::<Value>(&m);
                if m.is_err() {
                    continue;
                }
                let m = m.unwrap();
                parse_resp(m)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|m| m.try_into().ok())
                    .for_each(|m: BlockchainNotification| {
                        let notifications: Vec<_> = m.into();
                        println!("{:?}", notifications);
                    });
            }
        })
    }

    fn new(rpc: impl ToString, rest:impl ToString) -> Self {
        Self {
            websoket: rpc.to_string(),
            rest: rest.to_string(),
        }
    }

    async fn query_msg<T>(&self, msg:T) -> Result<T::Output, String>
    where T:Query {
        let p = msg.path(&self.rest);
        let r = reqwest::get(p).await.map_err(|e| e.to_string())?;
        let r = r.json::<Value>().await.map_err(|_| "Err2".to_string())?;
        println!("{}",r);
        serde_json::from_value::<T::Output>(r).map_err(|_| "Err3".to_string())

    }
}




#[tokio::test]
async fn test() {
    use crate::structs::query::wasm::{Cw20TokenInfo, QueryWasm};
    use crate::structs::query::auth::AccountInfo;
    let x = Blockchain::new(
        "wss://cosmos-rpc.publicnode.com:443/websocket", 
        "https://cosmos-rest.publicnode.com/"
    );
    let r = x.query_msg(AccountInfo {
        address:"cosmos15dk5pfsydl5rfx5f45pza95udjsu0y4jel7p6p".to_string()
    }).await;
    println!("{:?}", r);
        
    // let n = Notifier::new();
    // let _ = x.listen(n).await;
}
