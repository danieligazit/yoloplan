
use {
    std::str::FromStr,
    anyhow::{Result, Error},
    serde::{Serialize, Deserialize},
    crate::Event,
    arangors::{
        Connection, 
        AqlQuery, 
        Database, 
        Collection, 
        document::options::InsertOptions,
        client::reqwest::ReqwestClient,
    },
};

pub struct DAL{
    db: Database<ReqwestClient>,
    main_col: Collection<ReqwestClient>,
}


impl DAL{
    pub async fn new() -> anyhow::Result<DAL>{
        let conn = Connection::establish_jwt("http://localhost:8529", "root", "")
            .await
            .unwrap();        
        let db = conn.db("test").await?;
        let collection = db.collection("customers").await?;
        Ok(DAL{db: db, main_col: collection})
    }
    
    pub async fn upload(&self, event: Event) -> anyhow::Result<()>{
        let doc = self.main_col
            .create_document(event, InsertOptions::builder().return_new(true).build())
            .await?;
        // println!("{:#?}", doc);
        Ok(())
    }

    pub async fn identify(&self, event: Event) -> anyhow::Result<()>{
        Ok(())
    }

    // pub insert_document()
}