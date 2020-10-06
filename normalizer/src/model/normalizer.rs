use {
    async_trait::async_trait,
    std::error::Error,
    serde,
    serde::{Serialize},
}

#[async_trait]
pub trait<T> Normalizer{
    async fn normalize(&self, config: &Vec<u8>) -> NormalizeResult;
}

#[derive(Serialize, Debug)]
pub enum Normalized {
    Country{}
    MusicEvent{artists: Vec<String>, location: String},
}

impl ToQueue for Extracted {
    fn get_queue_name(&self) -> String {
        match self {
            Extracted::MusicEvent{artists: _, location: _} => "identifier.event.music".to_owned(),
        }
    }
}