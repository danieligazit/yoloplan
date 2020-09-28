use std::error::Error;
use scraper::Html;
use scraper::Selector;
use std::fmt;

pub struct MusicEvent{
    artists: Vec<String>,
    location: String
}

#[derive(Debug)]
struct MissingDataError;

impl fmt::Display for MissingDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There's missing data in the html element")
    }
}

impl Error for MissingDataError {
    fn description(&self) -> &str {
        "There's missing data in the html element"
    }
}

pub async fn get_url(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(body)
}

pub fn parse_bachtrack_html(body: String) -> Result<Vec<MusicEvent>, Box<dyn Error>> {
    let mut events = Vec::new();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("div.listing-shortform.listing-medium-1.li-shortform-premium").unwrap();

    for element in document.select(&selector) {
        match get_event_from_element(element) {
            Ok(event) => events.push(event),
            Err(_) => println!("could not parse element")
        }
    }

    for (pos, event) in events.iter().enumerate() {
        println!("Element at position {}: {:#?}", pos, event.artists);
    }
    Ok(events)
}

pub fn get_event_from_element(element: scraper::element_ref::ElementRef) -> Result<MusicEvent, Box<dyn Error>>{
    let event = MusicEvent{
        artists: get_artists_from_element(element)?,
        location: String::from("your house")
    };

    Ok(event)
}


pub fn get_artists_from_element(element: scraper::element_ref::ElementRef) -> Result<Vec<String>, Box<dyn Error>>{
    let selector = Selector::parse("div.listing-shortform-right div.listing-programme-simple").unwrap();
    let artistsElement = element.select(&selector).next().ok_or(MissingDataError)?;

    match get_artists_simple(artistsElement){
        Ok(artists) => return Ok(artists),
        Err(e) => return Ok(get_artists_detailed(element)?)
    }
}


pub fn get_artists_simple(element: scraper::element_ref::ElementRef) -> Result<Vec<String>, Box<dyn Error>>{
    println!("hey {} {:#?}", element.has_children(), element.text().collect::<String>()); //element.text()
    Ok(Vec::new())
}


pub fn get_artists_detailed(element: scraper::element_ref::ElementRef) -> Result<Vec<String>, Box<dyn Error>>{
    let mut artists = Vec::new();
    let selector = Selector::parse("span.composername").unwrap();
    
    for composerElement in element.select(&selector){
        let artist = String::from(composerElement.text().collect::<String>());
        
        if !artists.contains(&artist){
            artists.push(artist);
        }
    }

    Ok(artists)
}

pub fn main() {    
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(get_url("https://bachtrack.com/find-concerts/")).and_then(|body: String| {parse_bachtrack_html(body)}); 
}