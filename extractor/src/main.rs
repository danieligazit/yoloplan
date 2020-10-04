mod bachtrack;
mod model;
use nats::asynk as nats;
// use nats;
use std::collections::HashMap;
extern crate tokio;
use crate::model::extractor::Extractor;

#[tokio::main]
async fn main() {    
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // let extractor = 
    // rt.block_on(extractor.extract("https://bachtrack.com/find-concerts/"));

    // Using a threaded handler.

    let mut queue2extractor = HashMap::new();
    queue2extractor.insert("datasource.bachtrack", bachtrack::BachTrackExtractor{});
    
    
    let nc = nats::connect("127.0.0.1:4222").await.unwrap();

    for (queue, extractor) in queue2extractor {
        let subscriber: Stream = nc.subscribe(queue).await.unwrap();
        
        while let Some(message) = subscriber.next().await {
            let extracted = match extractor.extract(&message.data).await {
                Ok(k) => k,
                Err(e) => println!("Error extracting from datasource {}. err: {}", queue, e)
            };

            println!("finished");

        }
    }        
        // {
        //     Ok(sub) => sub,
        //     Err(e) => {
        //         println!("Couldn't subscribe to queue {}. err: {}", queue, e);
        //         continue;
        //     }
        // };

        
        // loop {
        //     let message = subscriber.next().await;
        //     let extracted = match extractor.extract(&msg.data).await {
        //         Ok(k) => k,
        //         Err(e) => println!("Error extracting from datasource {}. err: {}", queue, e)
        //     };
        //     println!("finished");
            

        //     // tokio::spawn(async move {
                
                
        //     // })
        // }
        

    

    
    
    // let sub = nc.subscribe(&subj).unwrap();
    // nc.publish(&subj, serde_json::to_vec(&m).unwrap()).unwrap();

    // let mut p2 = sub.iter().map(move |msg| {
    //     let p: model::MusicEvent = serde_json::from_slice(&msg.data).unwrap();
    // });

    // println!("received {:?}", p2.next().unwrap());

    // println!("{}", m.get_queue_name());
    // {
    //     Ok(_) => println!("Succesfully extracted message"),
    //     Err(e) => println!("Error exatrcting message. err={}", e)
    // }
}
