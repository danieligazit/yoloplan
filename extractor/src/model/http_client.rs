use {
    async_trait::async_trait,
    std::error::Error,
};

#[async_trait]
pub trait HttpClient:{
    async fn get(&self, url: &str) -> Result<String, Box<dyn Error>>;
}

#[derive(Copy, Clone)]
pub struct WebpageHttpClient{}
impl WebpageHttpClient {
    pub fn new() -> WebpageHttpClient {
        WebpageHttpClient {}
    }
}

#[async_trait]
impl HttpClient for WebpageHttpClient{
    async fn get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let body = reqwest::get(url)
            .await?
            .text()
            .await?;
    
        Ok(body)
    }
}

pub struct TestHttpClient<'a>{
    content: &'a str,
}

impl<'a> TestHttpClient<'a>{
    pub fn new(response_content: &str) -> TestHttpClient {
        TestHttpClient{content: response_content}       
    }
}

#[async_trait]
impl<'a> HttpClient for TestHttpClient<'a>{
    async fn get(&self, _url: &str) -> Result<String, Box<dyn Error>> {
        Ok(String::from(self.content))
    } 
}