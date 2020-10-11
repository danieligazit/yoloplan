extern crate tokio;
extern crate serde;
extern crate macros;
extern crate dotenv;

mod dal;

use {
    // nats::asynk as nats,
    // std::sync::Arc,    
    // std::error::Error,
    // serde_json::{Result, Value},
    serde::{Serialize, Deserialize},
    dal::*,
};

macros::schemafy!{
    "schema.json"
}
// const MAX_CONCURRENT_MESSAGES: usize = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    // let v: Event = serde_json::from_str(r#"{ 
    //     "title": "Amon Tobin - ISAM", 
    //     "time": "2021/10/10T20:00:00",
    //     "description": "Amon Tobin’s audiovisual spectacle ISAM took over the Concert Hall at Vivid LIVE 2012 in an audiovisual spectacle like no other.", 
    //     "price": 67
    // }"#)?;

    let neo4j_dal = neo4j::DAL::new().await?;

    Ok(())
}


// async fn setup_normalizer(){
//     use futures::stream::StreamExt;

//     let nc = nats::connect("127.0.0.1:4222").await.unwrap();
    
//     let subscriber = nc.subscribe("queue").await.unwrap();
//     let arc_nc = Arc::new(nc);
    
//     subscriber.for_each_concurrent(MAX_CONCURRENT_MESSAGES, move |message|{
//         let publisher = Arc::clone(&arc_nc);
        
//         async move{
            
//         }
//     }).await;
// }



