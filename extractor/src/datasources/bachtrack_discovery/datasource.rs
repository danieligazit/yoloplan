use {
    std::str,
    scraper::Html,
    scraper::Selector,
    async_trait::async_trait,
    crate::model::datasource::{Datasource, Extracted, ExtractResult},
    crate::model::http_client::{HttpClient},
};

pub const DS_NAME: &str = "datasource.bachtrack_discovery";

#[derive(Copy, Clone)]
pub struct DSBachTrackDiscovery<H: HttpClient>{
    pub http_client: H,
}

impl<H: HttpClient> DSBachTrackDiscovery<H>{
    pub fn new(http_client: H) -> DSBachTrackDiscovery<H>{
        DSBachTrackDiscovery{http_client: http_client}
    } 
}

#[async_trait]
impl<H: HttpClient + Send + Sync> Datasource for DSBachTrackDiscovery<H>{
    async fn extract(&self, configuration: &Vec<u8>) -> ExtractResult{
        let webpage: String = self.http_client.get(str::from_utf8(&configuration)?).await?;
        Ok(parse_bachtrack_html(&webpage)?)
    }
    
    fn get_name(&self) -> String{
        DS_NAME.to_owned()
    }
} 

pub fn parse_bachtrack_html(body: &str) -> ExtractResult {
    let mut items: Vec<Extracted> = Vec::new();

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
        items.push(Extracted::Configuration{ds_name: "datasource.bachtrack".to_owned(), value: listing_url.as_bytes().to_vec()});
    }

    Ok(items)
}


#[cfg(test)]
mod tests {
    use {
        super::{Datasource, Extracted},
        std::path::Path,
        std::error::Error,
        std::fs,
        tokio_test,
        crate::model::http_client::TestHttpClient,
    };

    const DISCOVERY_URL: &str = "https://bachtrack.com/find-concerts/";

    fn read_test_webpage() -> Result<String, Box<dyn Error>>{
        let base_path = std::env::var("CARGO_MANIFEST_DIR")?;
        let webpage_path = Path::new(&base_path).join("resources/tests/bachtrack.html");
        Ok(fs::read_to_string(webpage_path)?)
    }

    #[test]
    fn test_extracor() -> Result<(), Box<dyn Error>>{
        let webpage = read_test_webpage()?;
        
        let datasource = super::DSBachTrackDiscovery::new(TestHttpClient::new(&webpage));

        let configuration = DISCOVERY_URL.as_bytes().to_vec();
        let items = tokio_test::block_on(datasource.extract(&configuration))?;

        assert_eq!(items.len(), 50);
        assert_eq!(items[0], Extracted::Configuration{
            ds_name: "datasource.bachtrack".to_owned(),
            value: "https://bachtrack.com/concert-event/residenz-serenade-munich-residenz-solisten-die-residenz-hofkapelle-5-september-2019/318719".as_bytes().to_vec(),
        });
        Ok(())
    }
}
 