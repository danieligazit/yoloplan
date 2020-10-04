use super::Extracted;
use serde::{Deserialize, Serialize};

// #[destination_queue("event.music")]
#[derive(Serialize, Deserialize, Debug)]
pub struct MusicEvent{
    pub artists: Vec<String>,
    pub location: String
}

impl MusicEvent{
    pub fn new(artists: Vec<String>, location: String) -> MusicEvent{
        MusicEvent{artists, location}
    }
}

impl Extracted for MusicEvent {
    fn get_queue_name(&self) -> String {
        "extract.event.music".to_owned()
    }
}
