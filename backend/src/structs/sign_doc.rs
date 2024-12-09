use base64::prelude::*;
use bech32::{Bech32, Hrp};
use reqwest::StatusCode;
use secp256k1::{
    hashes::{ripemd160, sha256, Hash},
    Secp256k1,
};
use serde::{Deserialize, Serialize};

use super::error::map_err;

#[derive(Serialize, Deserialize)]
struct Fee {
    amount: Vec<String>,
    gas: String,
}
#[derive(Serialize, Deserialize)]

struct Value {
    data: String,
    signer: String,
}

#[derive(Serialize, Deserialize)]

struct Message {
    r#type: String,
    value: Value,
}

#[derive(Serialize, Deserialize)]

struct SignDoc {
    account_number: String,
    chain_id: String,
    fee: Fee,
    memo: String,
    msgs: Vec<Message>,
    sequence: String,
}

#[derive(Serialize, Deserialize)]
pub struct PubKey {
    r#type: String,
    value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Signature {
    pub signature: String,
    pub pub_key: PubKey,
    pub nonce: String,
}

impl Signature {
    pub fn verify(&self) -> Result<String, super::error::Error> {
        let nonce = BASE64_STANDARD.encode(&self.nonce);
        let sig = BASE64_STANDARD
            .decode(&self.signature)
            .map_err(map_err("Invalid signature", StatusCode::BAD_REQUEST))?;
        let pk = BASE64_STANDARD
            .decode(&self.pub_key.value)
            .map_err(map_err("Invalid pk", StatusCode::BAD_REQUEST))?;
        let hpk = sha256::Hash::hash(&pk);
        let hpk = ripemd160::Hash::hash(hpk.as_byte_array());
        let hpk = hpk.as_byte_array();
        let addr = bech32::encode::<Bech32>(Hrp::parse("cosmos").unwrap(), hpk)
            .map_err(map_err("Invalid pk", StatusCode::BAD_REQUEST))?
            .to_lowercase();

        let doc = SignDoc {
            account_number: "0".into(),
            chain_id: "".into(),
            fee: Fee {
                amount: vec![],
                gas: "0".into(),
            },
            sequence: "0".into(),
            memo: "".into(),
            msgs: vec![Message {
                r#type: "sign/MsgSignData".into(),
                value: Value {
                    data: nonce,
                    signer: addr.clone(),
                },
            }],
        };
        let ctx = Secp256k1::verification_only();
        let hash =
            sha256::Hash::hash(serde_json::to_string(&doc).unwrap().as_bytes()).to_byte_array();

        let msg = secp256k1::Message::from_digest(hash);
        let sig = secp256k1::ecdsa::Signature::from_compact(&sig)
            .map_err(map_err("Invalid signature", StatusCode::BAD_REQUEST))?;
        let pk = secp256k1::PublicKey::from_slice(&pk).unwrap();
        ctx.verify_ecdsa(&msg, &sig, &pk)
            .and_then(move |_| Ok(addr))
            .map_err(map_err("Invalid signagure", StatusCode::BAD_REQUEST))
    }
}
