use std::error::Error;
use std::str;
use scraper::Html;
use scraper::Selector;
use async_trait::async_trait;
use crate::model::{Extractor, Extracted, errors, MusicEvent, ExtractorResult};

pub struct BachTrackExtractor {}

#[async_trait]
impl Extractor for BachTrackExtractor{
    async fn extract(&self, configuration: &Vec<u8>) -> ExtractorResult{
        let webpage: String =  get_webpage(str::from_utf8(&configuration)?).await?;
        let events: Vec<Box<dyn Extracted>> = parse_bachtrack_html(webpage)?; 
        Ok(events)
    }
} 


pub async fn get_webpage(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(body)
}

pub fn parse_bachtrack_html(body: String) -> ExtractorResult {
    let mut events = Vec::new();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("div.listing-shortform.listing-medium-1.li-shortform-premium").unwrap();

    for element in document.select(&selector) {
        match get_event_from_element(element) {
            Ok(event) => {
                let extracted_event: Box<dyn Extracted> = Box::new(event);
                events.push(extracted_event);
            },
            Err(_) => println!("could not parse element")
        }
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
    let artists_element = element.select(&selector).next().ok_or(errors::MissingDataInHtmlError)?;

    match get_artists_simple(artists_element){
        Ok(artists) => return Ok(artists),
        Err(_e) => return Ok(get_artists_detailed(element)?)
    }
}


pub fn get_artists_simple(element: scraper::element_ref::ElementRef) -> Result<Vec<String>, Box<dyn Error>>{
    println!("hey {} {:#?}", element.has_children(), element.text().collect::<String>());
    Ok(Vec::new())
}


pub fn get_artists_detailed(element: scraper::element_ref::ElementRef) -> Result<Vec<String>, Box<dyn Error>>{
    let mut artists = Vec::new();
    let selector = Selector::parse("span.composername").unwrap();
    
    for composer_element in element.select(&selector){
        let artist = String::from(composer_element.text().collect::<String>());
        
        if !artists.contains(&artist){
            artists.push(artist);
        }
    }

    Ok(artists)
}
