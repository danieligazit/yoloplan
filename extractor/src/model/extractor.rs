use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Extractor {
    async fn extract(&self, config: &Vec<u8>) ->  Result<Vec<Box<dyn Extracted>>, Box<dyn Error>>;
}

pub trait Extracted {
    fn get_queue_name(&self) -> String;
}

pub type ExtractorResult = Result<Vec<Box<dyn Extracted>>, Box<dyn Error>>;