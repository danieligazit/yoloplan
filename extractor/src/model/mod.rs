use async_trait::async_trait;
use std::error::Error;

pub mod errors;

mod music_event;
pub use music_event::MusicEvent;

#[async_trait]
pub trait Extractor {
    async fn extract(&self, config: &str) ->  Result<Vec<Box<dyn Extracted>>, Box<dyn Error>>;
}

pub trait Extracted {
    fn get_queue_name(&self) -> String;
}

pub type ExtractorResult = Result<Vec<Box<dyn Extracted>>, Box<dyn Error>>;