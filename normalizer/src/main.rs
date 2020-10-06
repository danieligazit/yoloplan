use {
    nats::asynk as nats,
    std::sync::Arc,
};

const MAX_CONCURRENT_MESSAGES: usize = 100;

extern crate tokio;

#[tokio::main]
async fn main() {  
    tokio::join!(
        setup_identifier("identifier.event.music", bachtrack::MusicEventIdentifier{}),
    );

    // TODO:: switch to spawn task to use multi-thread (Tokio join does not allow multi-threading)
    // datasources
    //     .into_iter()
    //     .map(move |(queue, extractor)| {
    //         tokio::spawn(async move { 
    //             use crate::model::extractor::Extractor;
    //             setup_extractor(queue, extractor);
    //         }); 
    //     });
}

async fn setup_identifier<T: Identifier + Copy>(identifier_name: &str, extractor: T){
    use futures::stream::StreamExt;

    let nc = nats::connect("127.0.0.1:4222").await.unwrap();

    println!("listening to queue {}", identifier_name);

    let subscriber = nc.subscribe(datasource_name).await.unwrap();
    let arc_nc = Arc::new(nc);
    
    subscriber.for_each_concurrent(MAX_CONCURRENT_MESSAGES, move |message|{
        let publisher = Arc::clone(&arc_nc);
        
        async move{
            let extracted_items = match identifier.identify(&message.data).await{
                Ok(k) => k,
                Err(e) => {
                    println!("{} Error occured in the identifier logic. err: {}", datasource_name, e);
                    return;
                }
            };
            
        }
    }).await;
}
