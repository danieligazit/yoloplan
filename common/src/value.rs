use {
    serde::{Deserialize, Serialize},
    serde_json,
    std::net::{Ipv4Addr},
    std::fmt,
    std::collections::HashMap,
};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Value{  
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),
    
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    ISize(isize),
    USize(usize),
    Char(char),
    String(String),
    Vec(Vec<Value>),
    IP4(Ipv4Addr),
    Object(HashMap<String, Value>),
    Null,
}


impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        match *self {
            Value::String(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => Value::from(v),
            None => Value::Null
        }
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl<'a> From<Vec<Value>> for Value {
    fn from(f: Vec<Value>) -> Self {
        Value::Vec(f.into_iter().map(Into::into).collect())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match *self {
            String(ref value) => format_value(value, f),
            Bool(ref value) => format_value(value, f),
            F64(ref value) => format_value(value, f),
            F32(ref value) => format_value(value, f),
            ISize(ref value) => format_value(value, f),
            I64(ref value) => format_value(value, f),
            I32(ref value) => format_value(value, f),
            I16(ref value) => format_value(value, f),
            I8(ref value) => format_value(value, f),
            USize(ref value) => format_value(value, f),
            U64(ref value) => format_value(value, f),
            U32(ref value) => format_value(value, f),
            U16(ref value) => format_value(value, f),
            U8(ref value) => format_value(value, f),
            Char(ref value) => format_value(value, f),
            Object(ref value) => format_value_map(value, f),
            Vec(ref value) => format_value_list(value, f),
            IP4(ref value) => format_value(value, f),
            Null => format_value(&"null", f),
        }

    }
}

fn format_value<T>(value: &T, f: &mut fmt::Formatter) -> fmt::Result
where
    T: ToString,
{
    f.write_str(&value.to_string())
}

fn format_value_list<T>(values: &[T], f: &mut fmt::Formatter) -> fmt::Result
where
    T: std::fmt::Debug,
{   
    f.write_str(&format!("{:?}", values))
}



fn format_value_map<T>(values: &HashMap<String, T>, f: &mut fmt::Formatter) -> fmt::Result
where
    T: std::fmt::Debug,
{
    f.write_str(&format!("{:?}", values))
}

impl Value {
    pub fn into_serde_json_value(self) -> Result<serde_json::value::Value, serde_json::error::Error> {
        use self::Value::*;
        match self {
            String(ref value) => serde_json::value::to_value(value),
            Bool(ref value) => serde_json::value::to_value(value),
            F64(ref value) => serde_json::value::to_value(value),
            F32(ref value) => serde_json::value::to_value(value),
            ISize(ref value) => serde_json::value::to_value(value),
            I64(ref value) => serde_json::value::to_value(value),
            I32(ref value) => serde_json::value::to_value(value),
            I16(ref value) => serde_json::value::to_value(value),
            I8(ref value) => serde_json::value::to_value(value),
            USize(ref value) => serde_json::value::to_value(value),
            U64(ref value) => serde_json::value::to_value(value),
            U32(ref value) => serde_json::value::to_value(value),
            U16(ref value) => serde_json::value::to_value(value),
            U8(ref value) => serde_json::value::to_value(value),
            Char(ref value) => serde_json::value::to_value(value),
            Object(ref value) => serde_json::value::to_value(value),
            Vec(ref value) => serde_json::value::to_value(value),
            IP4(ref value) => serde_json::value::to_value(value),
            Null => serde_json::value::to_value(serde_json::value::Value::Null),
        }
    }
}

