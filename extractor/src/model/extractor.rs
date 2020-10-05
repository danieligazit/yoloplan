use async_trait::async_trait;
use std::error::Error;
use serde;
use serde::{Serialize};


#[async_trait]
pub trait Extractor{
    async fn extract(&self, config: &Vec<u8>) -> ExtractResult;
}

#[derive(Serialize, Debug)]
pub enum Extracted {
    MusicEvent{artists: Vec<String>, location: String},
}

pub trait ToQueue {
    fn get_queue_name(&self) -> String;
}

impl ToQueue for Extracted {
    fn get_queue_name(&self) -> String {
        match self {
            Extracted::MusicEvent{artists: _, location: _} => "extract.event.music".to_owned(),
        }
    }
}

pub type ExtractResult = Result<Vec<Extracted>, Box<dyn Error>>;