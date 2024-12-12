use serde::{Deserialize, Serialize};
use ts_rs::TS;



#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct ChatMessage {
    pub message:String,
    pub from:String,
    pub to:String,
    pub chat_id:String,
    pub date:u64,
}