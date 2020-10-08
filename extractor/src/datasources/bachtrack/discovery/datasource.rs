use {
    std::str,
    scraper::Html,
    scraper::Selector,
    async_trait::async_trait,
    crate::model::{Extracted, Datasource, ExtractResult, Configuration},
    crate::model::http_client::{HttpClient},
};

pub const DS_NAME: &str = "datasource.bachtrack_discovery";

#[derive(Copy, Clone)]
pub struct DS<H: HttpClient>{
    pub http_client: H,
}

impl<H: HttpClient> DS<H>{
    pub fn new(http_client: H) -> DS<H>{
        DS{http_client: http_client}
    } 
}

#[async_trait]
impl<H: HttpClient + Send + Sync> Datasource for DS<H>{
    async fn extract(&self, configuration: &Vec<u8>) -> ExtractResult{
        println!("extracting with configuration {}", str::from_utf8(&configuration)?);
        let webpage: String = self.http_client.get(str::from_utf8(&configuration)?).await?;
        Ok(parse_bachtrack_html(&webpage)?)
    }
    
    fn get_name(&self) -> String{
        DS_NAME.to_owned()
    }
} 

pub fn parse_bachtrack_html(body: &str) -> ExtractResult {
    let mut listings: Vec<Extracted> = Vec::new();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a.listing-more-info").unwrap();

    for element in document.select(&selector) {
        let listing_url = match element.value().attr("href"){
            Some(value) => value,
            None => {
                println!("Couldn't not find listing url");
                continue;
            }
        };
        listings.push(Extracted::Configuration(Configuration{ds_name: super::super::listing::DS_NAME.to_owned(), value: listing_url.to_string()}));
    }

    Ok(listings)
}
 