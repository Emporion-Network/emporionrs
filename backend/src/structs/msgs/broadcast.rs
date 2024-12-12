use base64::{prelude::BASE64_STANDARD, Engine};
use prost_reflect::DynamicMessage;
use reqwest::header::CONTENT_TYPE;
use secp256k1::{
    ecdsa::Signature,
    hashes::{sha256, Hash},
    PublicKey, Secp256k1,
};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};

use crate::{
    impl_encode,
    structs::{blockchain::DESCRIPTOR_POOL, msgs::msg_send::Coin},
    utils::key_pair_from_mnemonic,
};

use super::as_base64;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Mode {
    BroadcastModeSync,
}

#[derive(Serialize, Deserialize)]
struct BroadcastTxRequest {
    #[serde(serialize_with = "as_base64")]
    pub tx_bytes: Vec<u8>,
    pub mode: Mode,
}


#[derive(Serialize, Deserialize)]
struct TxDecodeRequest {
    #[serde(serialize_with = "as_base64")]
    pub tx_bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct SignMode {
    single: NestedSignMode,
}
#[derive(Serialize, Deserialize)]
pub struct NestedSignMode {
    pub mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignerInfo {
    pub mode_info: SignMode,
    pub sequence: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AuthInfo {
    signer_infos: Vec<SignerInfo>,
    fee: Fee,
}

#[derive(Serialize, Deserialize)]

pub struct Fee {
    amount: Vec<Coin>,
    gas_limit: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TxBody {
    pub messages: Vec<Message>,
    pub memo: String,
    pub timeout_height: u64,
    pub extension_options: Vec<String>,
    pub non_critical_extension_options: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub type_url: String,
    #[serde(serialize_with = "as_base64")]
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct SignDoc {
    #[serde(serialize_with = "as_base64")]
    body_bytes: Vec<u8>,
    #[serde(serialize_with = "as_base64")]
    auth_info_bytes: Vec<u8>,
    chain_id: String,
    account_number: u64,
}

#[derive(Serialize, Deserialize)]
struct TxRaw {
    #[serde(serialize_with = "as_base64")]
    body_bytes: Vec<u8>,
    #[serde(serialize_with = "as_base64")]
    auth_info_bytes: Vec<u8>,
    signatures: Vec<String>,
}

impl_encode!(BroadcastTxRequest, "cosmos.tx.v1beta1.BroadcastTxRequest");

impl_encode!(AuthInfo, "cosmos.tx.v1beta1.AuthInfo");

impl_encode!(TxBody, "custom.proto.TxBody");

impl_encode!(SignDoc, "cosmos.tx.v1beta1.SignDoc");

impl_encode!(TxRaw, "cosmos.tx.v1beta1.TxRaw");

impl_encode!(TxDecodeRequest, "cosmos.tx.v1beta1.TxDecodeRequest");

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Coin>,
}

impl_encode!(Msg, "cosmos.bank.v1beta1.MsgSend");


impl SignDoc {
    pub fn new(
        body: &TxBody,
        auth_info: &AuthInfo,
        chain_id: &str,
        account_number: u64,
    ) -> Option<Self> {
        Some(Self {
            body_bytes: body.encode()?,
            auth_info_bytes: auth_info.encode()?,
            chain_id: chain_id.to_string(),
            account_number,
        })
    }

    pub fn sign(self, signing_key: secp256k1::SecretKey) -> Option<TxRaw> {
        let sign_doc_bytes = &self.encode()?;
        let hash = sha256::Hash::hash(sign_doc_bytes).to_byte_array();
        let msg = secp256k1::Message::from_digest(hash);
        let secp = Secp256k1::new();
        let signature = secp.sign_ecdsa(&msg, &signing_key).serialize_compact();
        let signature = BASE64_STANDARD.encode(signature);
        Some(
            TxRaw {
                body_bytes: self.body_bytes,
                auth_info_bytes: self.auth_info_bytes,
                signatures: vec![signature],
            }
            .into(),
        )
    }
}

#[tokio::test]
async fn test() {
    let x = AuthInfo {
        signer_infos: vec![SignerInfo {
            mode_info: SignMode {
                single: NestedSignMode {
                    mode: "SIGN_MODE_DIRECT".to_string(),
                },
            },
            sequence: 0,
        }],
        fee: Fee {
            amount: vec![Coin {
                amount: "1000".to_string(),
                denom: "untrn".to_string(),
            }],
            gas_limit: 300000,
        },
    };


    let msg = Message {
        type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
        value: Msg {
            from_address: "neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2".to_string(),
            to_address: "neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2".to_string(),
            amount: vec![],
        }.encode().unwrap(),
    };

    let body = TxBody {
        messages: vec![msg],
        memo: "".to_string(),
        timeout_height: 0,
        extension_options: vec![],
        non_critical_extension_options: vec![],
    };

    let doc = SignDoc::new(&body, &x, "cosmoshub-4", 0).unwrap();

    let m = "clock post desk civil pottery foster expand merit dash seminar song memory figure uniform spice circle try happy obvious trash crime hybrid hood cushion";
    let x = key_pair_from_mnemonic(m).unwrap();
    let d = doc.sign(x.secret_key()).unwrap().encode().unwrap();
    let d = BroadcastTxRequest {
        tx_bytes: d,
        mode: Mode::BroadcastModeSync,
    };
    let d = serde_json::to_string(&d).unwrap();
    let c = reqwest::Client::new();
    let r = c
        .post("https://neutron-rest.publicnode.com/cosmos//tx/v1beta1/txs")
        .body(d);
    let resp = r.send().await.unwrap().json::<Value>().await.unwrap();
    println!("{:?}", resp);
}
