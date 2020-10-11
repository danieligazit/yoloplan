    
extern crate bolt_client;
use { 
    std::{
        collections::HashMap,
        convert::TryFrom,
        env,
        iter::FromIterator,
        error::Error,
    },
    serde::de::DeserializeOwned,
    serde_json::value,

    tokio::io::BufStream,
    tokio_util::compat::*,

    bolt_client::*,
    bolt_proto::{message::*, value::*, version::*, Message, Value},
};

type BoltClient = Client<Compat<BufStream<Stream>>>;

pub struct DAL{
    client: BoltClient,
}

impl DAL{
    pub async fn new() -> Result<DAL, Box<dyn std::error::Error>>{
        let domain: Option<&str> = None;
        let stream = Stream::connect(env::var("BOLT_ADDR")?, domain).await?;
        let stream = BufStream::new(stream).compat();

        let result = Client::new(stream, &[V4_1, V4_0, 0, 0]).await;

        let mut client = result.unwrap();

        client.hello(
            Some(Metadata::from_iter(vec![
                ("user_agent", "my-client-name/1.0"),
                ("scheme", "basic"),
                ("principal", &env::var("BOLT_USERNAME")?),
                ("credentials", &env::var("BOLT_PASSWORD")?),
            ]))).await?;

        Ok(DAL{client})
    }

    async fn run_query(&self, query: &str) -> Result<Message, Box<dyn Error>>{
        self.client.run_with_metadata("RETURN 1 as num;", None, None).await?
    }

    async fn parse_response<T: DeserializeOwned>(&self, response: Message) -> Result<T, Box<dyn Error>>{
        let result: Value = serde_json::de::from_str(&serde_json::to_string(&response)?)?;

        value::from_value::<T>(result).map_err(|e| {
            println!("Unable to parse response: {}", &e);
            From::from(e)
        })
    }

    
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();
//     println!("{}", env::var("BOLT_ADDR").unwrap());
//     let domain: Option<&str> = None;
//     let stream = Stream::connect(env::var("BOLT_ADDR").unwrap(), domain).await.unwrap();
//     let stream = BufStream::new(stream).compat();

//     let mut result = Client::new(stream, &[V4_1, V4_0, 0, 0]).await;

//     let mut client = result.unwrap();
    

//     let response: Message = client.hello(
//         Some(Metadata::from_iter(vec![
//             ("user_agent", "my-client-name/1.0"),
//             ("scheme", "basic"),
//             ("principal", &env::var("BOLT_USERNAME")?),
//             ("credentials", &env::var("BOLT_PASSWORD")?),
//         ]))).await?;
//     assert!(Success::try_from(response).is_ok());

//     Run a query on the server
//     let response = client.run_with_metadata("RETURN 1 as num;", None, None).await?;

//     println!("{:#?}", response);
//     // assert!(Success::try_from(response).is_ok());

//     // // Use PULL to retrieve results of the query, organized into RECORD messages
//     // // We get a (Message, Vec<Record>) returned from a PULL
//     // let pull_meta = Metadata::from_iter(vec![("n", 1)]);
//     // let (response, records) = client.pull(Some(pull_meta.clone())).await?;
//     // assert!(Success::try_from(response).is_ok());

//     // assert_eq!(records[0].fields(), &[Value::from(1)]);
    
//     // client.run_with_metadata("MATCH (n) DETACH DELETE n;", None, None).await?;
//     // client.pull(Some(pull_meta.clone())).await?;

//     // // Run a more complex query with parameters
//     // let params = Params::from_iter(vec![("name", "Rust")]);
//     // client.run_with_metadata(
//     //     "CREATE (:Client)-[:WRITTEN_IN]->(:Language {name: $name});",
//     //     Some(params), None).await?;
//     // client.pull(Some(pull_meta.clone())).await?;

//     // // Grab a node from the database and convert it to a native type
//     // client.run_with_metadata("MATCH (rust:Language) RETURN rust;", None, None).await?;
//     // let (response, records) = client.pull(Some(pull_meta.clone())).await?;
//     // assert!(Success::try_from(response).is_ok());
//     // let node = Node::try_from(records[0].fields()[0].clone())?;

//     // // Access properties from returned values
//     // assert_eq!(node.labels(), &[String::from("Language")]);
//     // assert_eq!(node.properties(),
//     //            &HashMap::from_iter(vec![(String::from("name"), Value::from("Rust"))]));

//     // // End the connection with the server
//     // client.goodbye().await?;

//     Ok(())
// }