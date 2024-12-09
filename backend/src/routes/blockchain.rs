use base64::prelude::*;
use futures::{SinkExt, StreamExt};
use prost_reflect::{DescriptorPool, DynamicMessage, FieldDescriptor, Kind, ReflectMessage};
use protobuf::{descriptor::FileDescriptorSet, SpecialFields};
use protobuf_parse::Parser;
use secp256k1::hashes::hex::DisplayHex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use tokio::fs;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::structs::notification::Notifier;


fn to_json(fd: FieldDescriptor, v: &prost_reflect::Value) -> Value {
    match fd.kind() {
        Kind::Double => Value::Number(Number::from_f64(v.as_f64().unwrap()).unwrap()),
        Kind::Float => Value::Number(Number::from_f64(v.as_f32().unwrap().into()).unwrap()),
        Kind::Int32 => Value::Number(Number::from_i128(v.as_i32().unwrap().into()).unwrap()),
        Kind::Int64 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Uint32 => Value::Number(Number::from_i128(v.as_u32().unwrap().into()).unwrap()),
        Kind::Uint64 => Value::Number(Number::from_i128(v.as_u64().unwrap().into()).unwrap()),
        Kind::Sint32 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Sint64 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Fixed32 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Fixed64 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Sfixed32 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Sfixed64 => Value::Number(Number::from_i128(v.as_i64().unwrap().into()).unwrap()),
        Kind::Bool => Value::Bool(v.as_bool().unwrap()),
        Kind::String => Value::String(v.as_str().unwrap().to_string()),
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
        Kind::Message(_) => panic!(""),
    }
}


fn parse_message(msg: DynamicMessage, ctx: &DescriptorPool) -> Value {
    let mut ret = Map::new();
    let msg = if let Some((type_url, decoder)) = msg.get_field_by_name("type_url").
    and_then(|v| v.as_str().map(|v| v[1..].to_string()))
    .and_then(|t| ctx.get_message_by_name(&t).map(|d| (t, d))) {
        let value = msg.get_field_by_name("value").unwrap().as_bytes().unwrap().to_vec();
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
                parse_message(v.as_message().unwrap().clone(), ctx),
            );
        } else if let Some(_) = v
            .as_list()
            .and_then(|l| l.iter().all(|v| v.as_message().is_some()).then(|| true))
        {
            let r = v
                .as_list()
                .unwrap()
                .iter()
                .map(|v| parse_message(v.as_message().unwrap().clone(), ctx))
                .collect::<Vec<_>>();
            ret.insert(f.json_name().to_string(), Value::Array(r));
        } else {
            ret.insert(f.json_name().to_string(), to_json(f, v));
        }
    });
    return Value::Object(ret);
}

fn parse_messages(tx: DynamicMessage, ctx: &DescriptorPool) -> Vec<Value> {
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
            let msg = parse_message(m.clone(), ctx);
            Some(msg)
            // let type_url = m.get_field_by_name("type_url")?;
            // let type_url = type_url.as_str().unwrap();
            // let decoder = ctx.get_message_by_name(&type_url[1..]);
            // if decoder.is_some() {
            //     let value = m.get_field_by_name("value")?;
            //     let value = value.as_bytes()?.to_vec();
            //     let message = DynamicMessage::decode(decoder?, value.as_slice()).ok()?;
            //     // let mut message = to_json(message);
            //     // message.as_object_mut().unwrap().insert("@type".to_string(),
            //     //  Value::String(type_url[1..].to_string())
            //     // );
            //     Some(message)
            // } else {
            //     println!("{}", type_url);
            //     None
            // }
        })
        .collect::<Vec<_>>();
    println!("{}", messages.len());
    messages
}

fn parse_resp(mut block_res: Value, ctx: &DescriptorPool) -> Option<Value> {
    let mut tx_result = block_res
        .get_mut("result")?
        .take()
        .get_mut("data")?
        .take()
        .get_mut("value")?
        .take()
        .get_mut("TxResult")?
        .take();
    let height = tx_result.get_mut("height")?.take();
    let height = height.as_str().to_owned()?;
    if tx_result
        .get_mut("result")?
        .take()
        .get_mut("data")
        .is_none()
    {
        return None;
    }
    let tx = tx_result.get_mut("tx")?;
    let tx_decoder = ctx.get_message_by_name("cosmos.tx.v1beta1.Tx").unwrap();
    let tx = BASE64_STANDARD.decode(tx.as_str().unwrap()).ok()?;
    let tx = DynamicMessage::decode(tx_decoder, tx.as_slice()).unwrap();
    println!("Ok  {:?}", parse_messages(tx, ctx));
    return None;
    // None
}

async fn get_files(path: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut entries = fs::read_dir(path).await.unwrap();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        let ftype = entry.file_type().await.unwrap();
        if ftype.is_dir() {
            let mut r = Box::pin(get_files(entry.path().to_str().unwrap())).await;
            res.append(&mut r);
            continue;
        }
        if entry.file_name().to_str().unwrap().ends_with(".proto") {
            res.push(entry.path().to_str().unwrap().to_string());
        }
    }
    res
}

async fn get_context() -> DescriptorPool {
    use protobuf::Message;
    let mut p = Parser::new();
    let f = get_files("proto").await;
    let r = p.include("proto").inputs(f).parse_and_typecheck().unwrap();
    let set = FileDescriptorSet {
        file: r.file_descriptors,
        special_fields: SpecialFields::new(),
    };
    let d = set.write_to_bytes().unwrap();
    DescriptorPool::decode(d.as_slice()).unwrap()
}

pub fn handle_blockchain_events() -> tokio::task::JoinHandle<()> {
    // let notifier = notifier.clone();
    tokio::spawn(async move {
        let (mut ws_stream, _) = connect_async("wss://cosmos-rpc.publicnode.com:443/websocket")
            .await
            .expect("Failed to connect");
        let msg = Message::text("{\"jsonrpc\": \"2.0\", \"method\": \"subscribe\", \"params\": [\"tm.event='Tx'\"], \"id\": 1 }");
        let _ = ws_stream.send(msg).await;
        let ctx = get_context().await;
        loop {
            let m = match ws_stream.next().await {
                Some(Ok(m)) => m.into_text(),
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
            parse_resp(m, &ctx);
        }
    })
}

