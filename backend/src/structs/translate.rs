use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
pub struct TranslateReq {
    pub translations: HashMap<String, String>,
}