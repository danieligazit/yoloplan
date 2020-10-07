use {
    std::str,
    scraper::Html,
    std::error::Error,
    chrono::{NaiveDateTime, Duration},
    scraper::Selector,
    async_trait::async_trait,
    crate::model::{Datasource, ExtractResult, EventTime, Extracted, MusicEvent, Person, Piece},
    crate::model::http_client::{HttpClient},
    crate::model::errors,
    regex::Regex,
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
    let (pieces, artists) = get_pieces_and_artists(&document);
    let description = get_description(&document);

    for start_time in get_event_times(&document)?{
        let end_time = match start_time.checked_add_signed(Duration::hours(DEFAULT_EVENT_LENGTH)){
            Some(time) => time,
            None => return Err(Box::new(errors::DateTimeCalculationError{}))
        };
        
        events.push(
            Extracted::MusicEvent(MusicEvent{
                time: EventTime{ 
                    start_time: start_time, 
                    end_time: end_time, 
                    local_timezone: true,
                },
                description: description.clone(),
                pieces: pieces.to_vec(),
                artists: artists.to_vec(),
            })
        );
    }
    Ok(events)
      
}
 
fn get_event_times(document: &Html) -> Result<Vec<chrono::NaiveDateTime>, Box<dyn Error>>{
    let mut times = Vec::new();
    for element in document.select(&Selector::parse("table#table_li_times").unwrap()){

        for time_element in element.select(&Selector::parse("tr").unwrap()){
            match parse_time(&time_element){
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

fn parse_time(time_element: &scraper::ElementRef) -> Result<NaiveDateTime, Box<dyn Error>>{    
    let date_strings = time_element.text().collect::<Vec<_>>();
    Ok(NaiveDateTime::parse_from_str(&date_strings.join(" "), DATE_TIME_FORMAT)?)
}

fn get_pieces_and_artists(document: &Html) -> (Vec<Piece>, Vec<Person>){
    let mut pieces = Vec::new();
    let mut artists: Vec<Person> = Vec::new();

    for element in document.select(&Selector::parse("table#table_listing-programme").unwrap()){
        for programme_element in element.select(&Selector::parse("tr").unwrap()){
            let artist_result = get_artist_name(&programme_element);
            
            match get_piece_name(&programme_element){
                Ok(piece_name) => {
                    let mut piece_artists = Vec::new();
                    
                    match artist_result {
                        Ok(ref artist_name) => piece_artists.push(Person{name: artist_name.to_owned()}),
                        Err(ref _e) => (),
                    }

                    pieces.push(Piece{
                       name: piece_name,
                       artists: piece_artists,
                    });
                },
                Err(e) => println!("Coudld not parse piece from html element. err: {}", e)
            }
            
            match artist_result{
                Ok(artist_name) => {
                    if !artists.iter().any(|artist| artist.name==artist_name) {
                        artists.push(Person{name: artist_name});
                    }
                },
                Err(e) => println!("Coudld not parse artist from html element. err: {}", e)
            }

        }
    }

    (pieces, artists)
}

fn get_artist_name(programme_element: &scraper::ElementRef) -> Result<String, Box<dyn Error>>{
    let re_arists_name = Regex::new(r"(?:Works by ){0,1}(.*) (?:.*)").unwrap();

    for element in programme_element.select(&Selector::parse("td:nth-child(1)").unwrap()){
        for capture in re_arists_name.captures_iter(&element.text().collect::<String>()) {
            if capture.len() != 2{
                return Err(Box::new(errors::RegexDidNotMatchError{}));
            }
            return Ok(capture[1].to_string());
        }
        return Err(Box::new(errors::RegexDidNotMatchError{}));
    }
    Err(Box::new(errors::MissingDataInHtmlError{}))
}

fn get_piece_name(programme_element: &scraper::ElementRef) -> Result<String, Box<dyn Error>>{
    for element in programme_element.select(&Selector::parse("td:nth-child(2)").unwrap()){
        let piece_name = element.text().collect::<String>();
        if piece_name != ""{
            return Ok(piece_name);
        }
    }
    Err(Box::new(errors::MissingDataInHtmlError{}))
}

fn get_description(document: &Html) -> String{
    for element in document.select(&Selector::parse("div.listing-description").unwrap()){ 
        return element.text().collect::<String>();
    }

    "".to_string()
}
