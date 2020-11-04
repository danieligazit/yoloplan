use super::Value;
use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};


#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Edit{
    time: DateTime<Utc>,
    old_value: Value,
}


#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Creation{
    time: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum HistoryItem{
    Edit(Edit),
    Creation(Creation),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Metadata{
    history: Vec<HistoryItem>,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata{history: Vec::new()}
    }
}