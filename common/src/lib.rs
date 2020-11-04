
use {
    serde::{Deserialize, Serialize},
    serde_json,
    std::net::{Ipv4Addr},
    std::fmt,
    std::collections::HashMap,
};


mod metadata;
mod simple_types;
mod value;

pub use value::Value;
pub use simple_types::SimpleTypes;
pub use metadata::Metadata;