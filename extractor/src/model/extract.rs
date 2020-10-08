use {
    serde::{Serialize, Deserialize},
    chrono::NaiveDateTime,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct EventTime {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub local_timezone: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct City {
    pub name: String,
    pub country: Country,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Country{
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Venue {
    pub name: String,
    pub address: String,
    pub city: City,
}

#[derive(Serialize,Deserialize,  Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Piece {
    pub name: String,
    pub artists: Vec<Person>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MusicEvent{
    pub artists: Vec<Person>,
    pub pieces: Vec<Piece>,
    // performers: Vec<Performer>,
    // venue: Venue,
    pub description: String,
    pub time: EventTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Configuration{
    pub ds_name: String, 
    pub value: String, 
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Extracted {
    MusicEvent(MusicEvent),
    Venue(Venue),
    City(City),
    Country(Country),
    Person(Person),
    Piece(Piece),
    Configuration(Configuration),
}


impl Extracted {
    pub fn get_queue_name(&self) -> String {
        match self {
            Extracted::MusicEvent(_) => "normalizer.event.music".to_owned(),
            Extracted::Venue(_) => "normalizer.venue".to_owned(),
            Extracted::Country(_) => "normalizer.country".to_owned(),
            Extracted::Person(_) => "normalizer.performer".to_owned(),
            Extracted::City(_) => "normalizer.city".to_owned(),
            Extracted::Piece(_) => "normalizer.piece".to_owned(),
            Extracted::Configuration(config) => config.ds_name.to_owned(),
        }
    }
}   