
use {
    // crate::Event,
    rincon_session::{ArangoSession, DatabaseSession, CollectionSession},
    rincon_core::api::{connector::Error as RinconError, datasource::DataSource},
    rincon_connector::http::JsonHttpConnector,
    tokio_core::reactor::Core,
    std::str::FromStr,
    anyhow::{Result, Error},
    serde::{Serialize, Deserialize},
    crate::Event,
};

pub struct DAL{
    col: CollectionSession<JsonHttpConnector>
}


fn create_session() -> Result<DatabaseSession<JsonHttpConnector>, RinconError> {
    let datasource = DataSource::from_str("http://localhost:8529")
        .expect("invalid URL for datasource")
        .with_basic_authentication("root", "");
    
    let mut core = Core::new()?;
    let connector = JsonHttpConnector::new(datasource, &core.handle())?;
    let session = ArangoSession::new(connector, core);
    let database = session.use_database_with_name("test");
    Ok(database)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Customer {
    name: String,
    age: u8,
}

impl DAL{
    pub async fn new() -> anyhow::Result<DAL>{
        let database = create_session().map_err(Error::msg)?;
        let collection = database.use_collection_with_name("customers");
        Ok(DAL{col: collection})
    }
    
    pub async fn upload(&self, event: Event) -> anyhow::Result<()>{
        let document_header = self.col.insert_document(event).map_err(Error::msg)?;
        Ok(())
    }

    pub async fn identify(&self, event: Event) -> anyhow::Result<()>{
        for 
        Ok(())
    }

    // pub insert_document()
}