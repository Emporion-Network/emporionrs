use base64::{prelude::BASE64_STANDARD, Engine};

pub mod broadcast;
pub mod ibc_recieve;
pub mod ibc_transfer;
pub mod msg_send;

#[macro_export]
macro_rules! exact_string {
    ($id:ident, $input:literal) => {
        fn $id<'de, D>(deserializer: D) -> Result<String, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let x = String::deserialize(deserializer)?;
            if (x == $input) {
                return Ok(x);
            } else {
                Err(serde::de::Error::custom("values are different"))
            }
        }
    };
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



fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S:  serde::Serializer,
{
    serializer.serialize_str(&BASE64_STANDARD.encode(&v))
}