use async_trait::async_trait;
use std::error::Error;
use serde;
use serde::{Serialize};


#[async_trait]
pub trait Datasource{
    async fn extract(&self, config: &Vec<u8>) -> ExtractResult;
    fn get_name(&self) -> String;
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Extracted {
    MusicEvent{artists: Vec<String>, location: String},
    Configuration{ds_name: String, value: Vec<u8>}
}

pub trait ToQueue {
    fn get_queue_name(&self) -> String;
}

impl ToQueue for Extracted {
    fn get_queue_name(&self) -> String {
        match self {
            Extracted::MusicEvent{artists: _, location: _} => "extract.event.music".to_owned(),
            Extracted::Configuration{ds_name, value: _} => ds_name.to_string(),
        }
    }
}

pub type ExtractResult = Result<Vec<Extracted>, Box<dyn Error>>;