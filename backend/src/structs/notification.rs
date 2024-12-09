use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use super::{blockchain::BlockchainNotification, chat_message::ChatMessage};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum Entity {
    User(String),
    Message{},
    Blockchain{},
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Data {
    Message(ChatMessage),
    BlockchainNotification(BlockchainNotification),
    Close {},
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct Notification {
    pub r#for: Entity,
    pub from:  Entity,
    pub data:  Data
}


pub type Sender = broadcast::Sender<Notification>;
pub type Receiver = broadcast::Receiver<Notification>;

pub struct Notifier {
    sender:broadcast::Sender<Notification>,
    reciver:broadcast::Receiver<Notification>
}

impl Notifier {
    pub fn new()->Self{
        let (ms, mr) = broadcast::channel::<Notification>(12);

        Self { 
            sender: ms, 
            reciver: mr 
        }
    }

    pub fn split(self)  -> (Sender, Receiver){
        (self.sender, self.reciver)
    }

}

impl Clone for Notifier {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone(), reciver: self.sender.subscribe() }
    }
}
