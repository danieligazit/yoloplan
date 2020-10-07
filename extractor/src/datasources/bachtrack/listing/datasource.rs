use {
    std::str,
    scraper::Html,
    std::error::Error,
    chrono::{NaiveDateTime, Duration},
    scraper::Selector,
    async_trait::async_trait,
    crate::model::{Datasource, ExtractResult, EventTime, Extracted, MusicEvent},
    crate::model::http_client::{HttpClient},
    crate::model::errors,
};

pub const DS_NAME: &str = "datasource.bachtrack_listing";
const DEFAULT_EVENT_LENGTH: i64 = 2;

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
        let webpage: String = self.http_client.get(str::from_utf8(&configuration)?).await?;
        Ok(parse_bachtrack_html(&webpage)?)
    }
    
    fn get_name(&self) -> String{
        DS_NAME.to_owned()
    }
} 

fn parse_bachtrack_html(body: &str) -> ExtractResult {
    let mut events = Vec::new();
    
    let document = Html::parse_document(&body);

    for start_time in get_event_times(document)?{
        let end_time = match start_time.checked_add_signed(Duration::hours(DEFAULT_EVENT_LENGTH)){
            Some(time) => time,
            None => return Err(Box::new(errors::DateTimeCalculationError{}))
        };

        events.push(
            Extracted::MusicEvent(MusicEvent{
                time: EventTime{ 
                    start_time: start_time, 
                    end_time: end_time, 
                    local_timezone: true
                }
            })
        );
    }
    Ok(events)
      
}
 
fn get_event_times(document: Html) -> Result<Vec<chrono::NaiveDateTime>, Box<dyn Error>>{
    let mut times = Vec::new();
    let selector = Selector::parse("table#table_li_times").unwrap();

    for element in document.select(&selector){
        let tr_selector = Selector::parse("tr").unwrap();

        for time_element in element.select(&tr_selector){
            match parse_time(time_element){
                Ok(time) => times.push(time),
                Err(e) => {
                    println!("Couldn't parse time from html element. err: {}", e);
                    continue;
                }
            }
        }
    }

    Ok(times)
}

const DATE_TIME_FORMAT: &str = "%A %d %B %Y %H:%M";

fn parse_time(time_element: scraper::ElementRef) -> Result<NaiveDateTime, Box<dyn Error>>{    
    let date_strings = time_element.text().collect::<Vec<_>>();
    Ok(NaiveDateTime::parse_from_str(&date_strings.join(" "), DATE_TIME_FORMAT)?)
}