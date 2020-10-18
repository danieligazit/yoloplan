    
extern crate bolt_client;
use { 
    std::{
        collections::HashMap,
        convert::TryFrom,
        env,
        fmt,
        iter::FromIterator,
        error::Error,
    },
    serde::Serialize,
    serde_json,

    tokio::io::BufStream,
    tokio_util::compat::*,

    bolt_client::*,
    bolt_proto::{message::{Record}, value as bolt_value, version::*, Message, Value, error},
};

type BoltClient = Client<Compat<BufStream<Stream>>>;

pub struct DAL{
    client: BoltClient,
}

#[derive(Debug)]
pub struct DALCommunciationError;

impl fmt::Display for DALCommunciationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to communicate to DAL resource")
    }
}

impl Error for DALCommunciationError {
    fn description(&self) -> &str {
        "Failed to communicate to DAL resource"
    }
}

fn serde2bolt_value(s_value: serde_json::Value) -> Value {
    match s_value {
        serde_json::Value::String(s) => Value::from(s),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64(){
                return Value::from(i);
            } else if let Some(f) = n.as_f64(){
                return Value::from(f);
            }
            Value::Null
        },
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::from(b),
        serde_json::Value::Array(a) => {
            let mut transformed = Vec::new();
            for item in a {
                transformed.push(serde2bolt_value(item));
            }
            Value::from(transformed)
        },
        serde_json::Value::Object(a) => {
            let mut transformed: HashMap<Value, Value> = HashMap::new();
            for (key, value) in a {
                transformed.insert(Value::from(key), serde2bolt_value(value));
            }
            Value::Null
        },
        _ => Value::Null,
        
    }
}

// fn bolt_value2serde(b_value: bolt_value) -> Value {
//     match s_value {
//         bolt_value::Value::String(s) => Value::from(s),
//         serde_json::Value::Number(n) => {
//             if let Some(i) = n.as_i64(){
//                 return Value::from(i);
//             } else if let Some(f) = n.as_f64(){
//                 return Value::from(f);
//             }
//             Value::Null
//         },
//         serde_json::Value::Null => Value::Null,
//         serde_json::Value::Bool(b) => Value::from(b),
//         serde_json::Value::Array(a) => {
//             let mut transformed = Vec::new();
//             for item in a {
//                 transformed.push(serde2bolt_value(item));
//             }
//             Value::from(transformed)
//         },
//         serde_json::Value::Object(a) => {
//             let mut transformed: HashMap<Value, Value> = HashMap::new();
//             for (key, value) in a {
//                 transformed.insert(Value::from(key), serde2bolt_value(value));
//             }
//             Value::Null
//         },
//         _ => Value::Null,
        
//     }
// }


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

    async fn run_query(&mut self, query: &str, params: Option<Params>) -> Result<Vec<Record>, Box<dyn Error>>{
        self.client.run_with_metadata(query, params, None).await?;
        
        let pull_meta = Metadata::from_iter(vec![("n", 1)]);
        let (response, records) = self.client.pull(Some(pull_meta.clone())).await?;

        let metadata = match response {
            Message::Success(success) => success,
            _ => {
                println!("meta {:#?}", response);
                return Err(Box::new(DALCommunciationError));
            }
        };
        
        Ok(records)
    }

    pub async fn load(&mut self, object_type: &str, values: HashMap<&str, serde_json::Value>) -> Result<(), Box<dyn Error>>{
        let mut params_vec = Vec::new();
        let mut query_values = Vec::new();

        for (field_name, field_value) in values {
            params_vec.push((field_name, serde2bolt_value(field_value)));
            query_values.push(format!("{}: ${}", field_name, field_name));
        }

        let params = Params::from_iter(params_vec); 
        let query = format!("CREATE (:{} {{{}}});", object_type, query_values.join(","));
        let records = self.run_query(&query, Some(params)).await;

        println!("records {:#?}", records);

        Ok(())
    }

    pub async fn identify(&mut self, object_type: &str, identifier_values: Vec<(&str, serde_json::Value, &str)>) -> Result<(), Box<dyn Error>>{
        let mut params_vec = Vec::new();
        let mut query_values = Vec::new();

        for (field, value, matching_method) in identifier_values {
            query_values.push(format!("{}: ${}", field, field));
            params_vec.push((field, serde2bolt_value(value)));
        }

        let params = Params::from_iter(params_vec); 
        let query = format!("MATCH (a:{} {{{}}}) RETURN a;", object_type, query_values.join(","));
        
        let records = self.run_query(&query, Some(params)).await;

        Ok(())
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