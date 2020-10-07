use {
    async_trait::async_trait,
    serde::{Serialize},
    chrono::NaiveDateTime,
};

#[derive(Serialize, Debug, PartialEq)]
pub struct EventTime {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub local_timezone: bool,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct City {
    pub name: String,
    pub country: Country,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Country{
    pub name: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Venue {
    pub name: String,
    pub address: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Performer {
    pub name: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MusicEvent{
    // artists: Vec<String>,
    // performers: Vec<Performer>,
    // venue: Venue,
    pub time: EventTime,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Extracted {
    MusicEvent(MusicEvent),
    Venue(Venue),
    City(City),
    Country(Country),
    Performer(Performer),
    Configuration{ds_name: String, value: Vec<u8>},
}


impl Extracted {
    pub fn get_queue_name(&self) -> String {
        match self {
            Extracted::MusicEvent(_) => "normalizer.event.music".to_owned(),
            Extracted::Venue(_) => "normalizer.venue".to_owned(),
            Extracted::Country(_) => "normalizer.country".to_owned(),
            Extracted::Performer(_) => "normalizer.performer".to_owned(),
            Extracted::City(_) => "normalizer.city".to_owned(),
            Extracted::Configuration{ds_name, value: _} => ds_name.to_owned(),
        }
    }
}   