use serde::{Deserialize, Serialize};



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub message:String,
    pub from:String,
    pub to:String,
    pub chat_id:String,
    pub date:u64,
}