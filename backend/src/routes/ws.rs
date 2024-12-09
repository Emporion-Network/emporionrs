use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures::{
    stream::{SplitSink, SplitStream, StreamExt}, SinkExt
};

use crate::structs::{
    jwt::{Token, TokenResp},
    notification::{Data, Entity, Notification, Notifier, Receiver, Sender},
};

#[derive(Clone)]
pub struct WsParams {
    pub notifier: Notifier,
    pub jwt_secret: String,
}

pub fn ws(params: &(impl Into<WsParams> + Clone)) -> Router {
    let params: WsParams = params.clone().into();
    Router::new().route("/ws", get(handler)).with_state(params)
}

pub async fn handler(State(params): State<WsParams>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|soket| handle_socket(soket, params))
}

async fn handle_socket(mut socket: WebSocket, params: WsParams) {
    let token: Token = if let Some(Ok(Message::Text(message))) = socket.recv().await {
        let message = serde_json::from_str::<TokenResp>(&message);
        if message.is_err() {
            let _ = socket.close().await;
            return;
        }
        let message = message.unwrap().try_into_token(&params.jwt_secret);
        if message.is_err() {
            let _ = socket.close().await;
            return;
        }
        message.unwrap()
    } else {
        let _ = socket.close().await;
        return;
    };
    let (sender, receiver) = socket.split();
    let (n_sender, n_reciver) = params.notifier.split();
    tokio::spawn(send_message(sender, n_reciver, token.clone()));
    tokio::spawn(got_message(receiver, n_sender, token));
}

async fn got_message(mut receiver: SplitStream<WebSocket>, n_sender: Sender, from: Token) {
    while let Some(Ok(Message::Text(message))) = receiver.next().await {
        let message = serde_json::from_str::<Notification>(&message);
        if message.is_err() {
            break;
        }
        let notification = message.unwrap();
        if notification.from != Entity::User(from.addr.clone()) {
            break;
        }
        if let Entity::User(_) = notification.r#for {
            break;
        }
        if let Err(_) = n_sender.send(notification) {
            break;
        };
    }
    let _ = n_sender.send(Notification {
        r#for:Entity::User(from.addr),
        from:Entity::Message {  },
        data: Data::Close {  },
    });
}

async fn send_message(
    mut sender: SplitSink<WebSocket, Message>,
    mut n_reciver: Receiver,
    to: Token,
) {
    while let Ok(notification) = n_reciver.recv().await {
        if notification.r#for != Entity::User(to.addr.clone()) {
            continue;
        };
        match (&notification.data, &notification.from) {
            (Data::Close {}, Entity::User(_)) => (),
            (Data::Close {}, _) => {
                let _ = sender.close().await;
                return ;
            },
            _ => ()
        };
        let notification = serde_json::to_string(&notification);
        if notification.is_err() {
            return;
        }
        let notification = notification.unwrap();
        if sender.send(Message::Text(notification)).await.is_err() {
            let _ = sender.close().await;
            return;
        };
    }
}
