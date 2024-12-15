use std::sync::Arc;

use qdrant_client::Qdrant;

#[derive(Clone)]
pub struct Db {
    client:Arc<Qdrant>
}

impl Db {
    pub fn new()->Self{
        Db {
            client: Arc::new(Qdrant::from_url("http://localhost:6334").build().unwrap())
        }
    }

}

#[tokio::test]

async fn test(){
    let x = Db::new();
    let c = x.clone();
    tokio::spawn(async move {
        println!("{:?}", c.client.health_check().await);
    });
    println!("{:?}",x.client.health_check().await);
}