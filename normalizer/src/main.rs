extern crate tokio;
extern crate serde;
extern crate schema;
extern crate dotenv;

mod dal;
pub mod model;

use {
    serde::{Serialize, Deserialize},
    dal::*,
};



schema::schemafy!{
    "schema.json"
}
// const MAX_CONCURRENT_MESSAGES: usize = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();
    let v: Event = serde_json::from_str(r#"{ 
        "title": "Amon Tobin - ISAM", 
        "time": "2021/10/10T20:00:00",
        "description": "Amon Tobin’s audiovisual spectacle ISAM took over the Concert Hall at Vivid LIVE 2012 in an audiovisual spectacle like no other.", 
        "price": 67
    }"#)?;

    let db = arangodb::DAL::new().await?;
    db.upload(v).await?;
    Ok(())
}

