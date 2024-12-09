use crate::structs::notification::{Data, Entity, Notification, Notifier};

pub fn handle_chat_messages(notifier: &Notifier) {
    let notifier = notifier.clone();
    tokio::spawn(async move {
        let (s, mut r) = notifier.split();
        loop {
            let notification = r.recv().await;
            if notification.is_err() {
                continue;
            };
            let notification = notification.unwrap();
            if notification.r#for != (Entity::Message {}) {
                continue;
            }
            let data = match notification.data {
                Data::Message(data) => data,
                _ => {
                    let _ = s.send(Notification {
                        r#for: notification.from,
                        r#from: Entity::Message {},
                        data: Data::Close {},
                    });
                    continue;
                }
            };
            if Entity::User(data.from.clone()) != notification.from {
                let _ = s.send(Notification {
                    r#for: notification.from,
                    r#from: Entity::Message {},
                    data: Data::Close {},
                });
                continue;
            };

            println!("Save to db: {:?}", &data);

            let _ = s.send(Notification {
                from:Entity::Message{},
                r#for:Entity::User(data.to.clone()),
                data: Data::Message(data),
            });
            
        }
    });
}
