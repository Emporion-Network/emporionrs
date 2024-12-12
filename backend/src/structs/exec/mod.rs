use serde_json::Value;

use super::blockchain::DESCRIPTOR_POOL;

pub mod broadcast;

pub fn encode_json(v:Value, path:&str) -> Option<Vec<u8>>{
    use prost_reflect::prost::Message;
    let message_descriptor= DESCRIPTOR_POOL.get_message_by_name(path)?;
    let json = serde_json::to_string(&v).ok()?;
    let mut deserializer = serde_json::Deserializer::from_str(&json);
    let dynamic_message = prost_reflect::DynamicMessage::deserialize(message_descriptor, &mut deserializer).unwrap(); //.ok()?;
    deserializer.end().ok()?;
    Some(dynamic_message.encode_to_vec())
}

#[macro_export]
macro_rules! impl_encode {
    ($id:ident, $input:literal) => {
        impl $id {
            fn encode(&self) -> Option<Vec<u8>> {
                use prost_reflect::prost::Message;
                let message_descriptor= DESCRIPTOR_POOL.get_message_by_name($input)?;
                let json = serde_json::to_string(&self).ok()?;
                let mut deserializer = serde_json::Deserializer::from_str(&json);
                let dynamic_message = prost_reflect::DynamicMessage::deserialize(message_descriptor, &mut deserializer).unwrap(); //.ok()?;
                deserializer.end().ok()?;
                Some(dynamic_message.encode_to_vec())
            }
        }
    };
}