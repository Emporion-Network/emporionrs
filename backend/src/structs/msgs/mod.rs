use base64::{prelude::BASE64_STANDARD, Engine};

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


pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S:  serde::Serializer,
{
    serializer.serialize_str(&BASE64_STANDARD.encode(&v))
}