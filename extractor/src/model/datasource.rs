use {
    async_trait::async_trait,
    super::Extracted,
    std::error::Error,
};

#[async_trait]
pub trait Datasource{
    async fn extract(&self, config: &Vec<u8>) -> ExtractResult;
    fn get_name(&self) -> String;
}

pub type ExtractResult = Result<Vec<Extracted>, Box<dyn Error>>;