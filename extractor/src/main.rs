mod bachtrack;
mod model;
use nats;

#[macro_use]
extern crate macros;

pub fn main() {    
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // let extractor = bachtrack::BachTrackExtractor{};
    // rt.block_on(extractor.extract("https://bachtrack.com/find-concerts/"));
    

    let m = model::MusicEvent::new(vec!["hi".to_string()], "hey".to_string());

    let nc = nats::connect("127.0.0.1:4222").unwrap();
    let subj = nc.new_inbox();

    let sub = nc.subscribe(&subj).unwrap();
    nc.publish(&subj, serde_json::to_vec(&m).unwrap()).unwrap();

    let mut p2 = sub.iter().map(move |msg| {
        let p: model::MusicEvent = serde_json::from_slice(&msg.data).unwrap();
        p
    });

    println!("received {:?}", p2.next().unwrap());

    // println!("{}", m.get_queue_name());
}
