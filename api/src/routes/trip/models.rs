use {
    serde::{
        Serialize, Deserialize,
    }
};


/// Configuration provided by the API user for finding a complete trip.
/// Based in this information, the system will need to find:
///  - Flights
///  - Hotels
///  - Events
#[derive(Serialize, Deserialize)]
pub struct FindTripConfig {
    pub from_iso_datetime: String,
    pub to_iso_datetime: String
}