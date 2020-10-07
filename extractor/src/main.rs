mod datasources;
mod model;
use {
    nats::asynk as nats,
    std::sync::Arc,
    crate::model::datasource::{Datasource, ToQueue},
    crate::model::http_client::WebpageHttpClient,
};

const MAX_CONCURRENT_MESSAGES: usize = 100;

extern crate tokio;

#[tokio::main]
async fn main() {  
    tokio::join!(
        setup_datasource(datasources::bachtrack_discovery::DSBachTrackDiscovery::new(
            WebpageHttpClient::new()
        )),
    );

    // TODO:: switch to spawn task to use multi-thread (Tokio join does not use multi-threading)
    // datasources
    //     .into_iter()
    //     .map(move |(queue, extractor)| {
    //         tokio::spawn(async move { 
    //             use crate::model::extractor::Extractor;
    //             setup_extractor(queue, extractor);
    //         }); 
    //     });
}

async fn setup_datasource<T: Datasource + Copy>(datasource: T){
    use futures::stream::StreamExt;

    let datasource_name = datasource.get_name();
    let nc = nats::connect("127.0.0.1:4222").await.unwrap();

    println!("listening to queue {}", datasource_name);

    let subscriber = nc.subscribe(&datasource_name).await.unwrap();
    let arc_nc = Arc::new(nc);
    
    subscriber.for_each_concurrent(MAX_CONCURRENT_MESSAGES, move |message|{
        let publisher = Arc::clone(&arc_nc);
        async move{

            let datasource_name = datasource.get_name();

            let extracted_items = match datasource.extract(&message.data).await{
                Ok(k) => k,
                Err(e) => {
                    println!("{} Error occured in the extract logic. err: {}", &datasource_name, e);
                    return;
                }
            };
            
            for item in extracted_items {
                let message = match serde_json::to_string(&item){
                    Ok(msg) => msg,
                    Err(e) => {
                        println!("{}: Error serializing the extracted item from into a message. err: {}, item:{:?}", &datasource_name, e, item);
                        continue;
                    }
                };
                
                let destination_queue = item.get_queue_name();
                match publisher.publish(&destination_queue, &message).await {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}  Error publishing a message to the '{}' queue. err: {}, message: {}", &datasource_name, destination_queue, e, message);
                        continue;
                    }
                };
            }
        }
    }).await;
}
