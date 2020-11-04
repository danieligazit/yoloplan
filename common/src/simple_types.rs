
use {
    serde::{Deserialize, Serialize},
    serde_json,
    std::net::{Ipv4Addr},
    std::fmt,
    std::collections::HashMap,
};


#[serde(rename = "simpleTypes")]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum SimpleTypes{  
    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "i8")]
    I8,

    #[serde(rename = "i16")]
    I16,

    #[serde(rename = "i32")]
    I32,

    #[serde(rename = "u64")]
    U64,

    #[serde(rename = "u32")]
    U32,

    #[serde(rename = "u16")]
    U16,

    #[serde(rename = "u8")]
    U8,

    #[serde(rename = "i64")]
    I64,

    #[serde(rename = "f64")]
    F64,

    #[serde(rename = "f32")]
    F32,

    #[serde(rename = "isize")]
    ISize,

    #[serde(rename = "usize")]
    USize,

    #[serde(rename = "char")]
    Char,

    #[serde(rename = "string")]
    String,

    #[serde(rename = "vec")]
    Vec,

    #[serde(rename = "ip4")]
    IP4,

    #[serde(rename = "object")]
    Object,

    #[serde(rename = "null")]
    Null,
}
