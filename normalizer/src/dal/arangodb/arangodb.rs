
use {
    std::str::FromStr,
    anyhow::{Result, Error},
    serde::{Serialize, Deserialize},
    crate::Event,
    arangors::{
        Connection, 
        AqlQuery, 
        Database, 
        Document,
        Collection, 
        document::options::InsertOptions,
        client::reqwest::ReqwestClient,
    },
    std::collections::HashMap,
    std::concat,
};

pub struct DAL{
    db: Database<ReqwestClient>,
    main_col: Collection<ReqwestClient>,
    main_col_name: String
}


impl DAL{
    pub async fn new() -> anyhow::Result<Self>{
        let conn = Connection::establish_jwt("http://localhost:8529", "root", "")
            .await?;        
        let db = conn.db("test").await?;
        let collection_name = Event::get_name();
        let collection = Self::ensure_collection(&collection_name, &db).await?;
        
        Ok(DAL{db: db, main_col: collection, main_col_name: collection_name.to_owned()})
    }
    
    
    async fn ensure_collection(collection_name: &str, db: &Database<ReqwestClient>) -> anyhow::Result<Collection<ReqwestClient>>{
        for collection_info in db.accessible_collections().await?{
            if collection_info.name == collection_name{
                return Ok(db.collection(&collection_name).await?);
            }
        }
        Ok(db.create_collection(&collection_name).await?)
    }

    pub async fn upload(&self, event: Event) -> anyhow::Result<()>{
        let doc = self.main_col
            .create_document(event, InsertOptions::builder().return_new(true).build())
            .await?;
        // println!("{:#?}", doc);
        Ok(())
    }

    pub async fn identify(&self, event: Event) -> anyhow::Result<Vec<Event>>{
        let mut query_lines = vec![("FOR u IN ".to_owned() + &Event::get_name())];
        let mut vars: HashMap<&str, serde_json::value::Value> = HashMap::new();

        for (field, value, method) in event.get_identifier_values()?{
            query_lines.push(format!("FILTER u.{}==@{}", field, field));
            vars.insert(field, value.into_serde_json_value()?);
        }
    
        query_lines.push("RETURN u".to_owned());

        let result: Vec<Document<Event>> = self.db
            .aql_bind_vars(&query_lines.join(" "), vars)
            .await?;
             
        Ok(result
            .into_iter()
            .map(|x| x.document)
            .collect())
    }

    // pub insert_document()
}