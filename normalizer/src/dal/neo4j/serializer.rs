

use serde::{ser, Serialize};
use super::error::{Error, Result, ErrorCode};
use crate::model::SerializerType;

pub struct Serializer {
    // This string starts empty and JSON is appended as values are serialized.
    output: String,
    current: String,
    // Specifies wether current seralized value is a field (if so, it skips the quotation marks)
    is_key: bool,
    should_pop: bool,
    previous_type: SerializerType,
    previous_keys: Vec<String>,
}

enum SerializeTypes {

}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        current: String::new(),
        is_key: false,
        should_pop: false,
        previous_type: SerializerType::None,
        previous_keys: Vec::new(),
    };
    value.serialize(&mut serializer)?;
    serializer.commit();
    Ok(serializer.output)
}

impl Serializer {
    fn commit(&mut self){
        self.output += &self.current;
        self.current.clear();
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.current += if v { "true" } else { "false" };
        self.previous_type = SerializerType::Bool;
        Ok(())
    }

    // JSON does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same. Other formats, especially compact binary
    // formats, may need independent logic for the different sizes.
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.previous_type = SerializerType::I8;
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.previous_type = SerializerType::I16;
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.previous_type = SerializerType::I32;
        self.serialize_i64(i64::from(v))
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.previous_type = SerializerType::I64;
        self.current += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.previous_type = SerializerType::U8;
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.previous_type = SerializerType::U16;
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.previous_type = SerializerType::U32;
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.previous_type = SerializerType::U64;
        self.current += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.previous_type = SerializerType::F32;
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.previous_type = SerializerType::F64;
        self.current += &v.to_string();
        Ok(())
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        self.previous_type = SerializerType::Char;
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.previous_type = SerializerType::Str;
        if self.is_key{
            self.previous_keys.push(v.to_string());
            self.should_pop = true;
            self.current += &format!("`{}`", self.previous_keys.join("."));
        } else {
            self.current += "\"";
            self.current += v;
            self.current += "\"";
        }
        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.previous_type = SerializerType::Bytes;
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        self.previous_type = SerializerType::None;
        self.serialize_unit()
    }


    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.previous_type = SerializerType::Some;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.previous_type = SerializerType::Unit;
        self.current += "null";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.previous_type = SerializerType::UnitStruct;
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.previous_type = SerializerType::UnitVariant;
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.previous_type = SerializerType::NewTypeStruct;
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.previous_type = SerializerType::NewTypeVariant;

        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":";
        value.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    // Now we get to the serialization of compound types.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.previous_type = SerializerType::Seq;
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.previous_type = SerializerType::Tuple;
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.previous_type = SerializerType::TupleStruct;
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.previous_type = SerializerType::TupleVariant;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.previous_type = SerializerType::Map;
        if self.previous_keys.len() == 0 {
            self.current += "{";
        }
        Ok(self) 
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.previous_type = SerializerType::Struct;
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.previous_type = SerializerType::StructVariant;
        self.current += "{";
        variant.serialize(&mut *self)?;
        self.current += ":{";
        Ok(self)
    }
}


impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {Ok(())}


    fn end(self) -> Result<()> {
        Ok(())}
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {Ok(())}

    fn end(self) -> Result<()> {Ok(())}
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {Ok(())}
    
    fn end(self) -> Result<()> {Ok(())}
}


impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,{Ok(())}

    fn end(self) -> Result<()> {Ok(())}
}


impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.current.ends_with('{') {
            self.current += ",";
        }
        self.is_key = true;
        let res = key.serialize(&mut **self);
        self.is_key = false;
        self.current += ":";
        res
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.current += "}";
        self.commit();
        Ok(())
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V
    ) -> Result<()>
    where
        K: Serialize,
        V: Serialize,
    {
        
        self.serialize_key(key);
        
        match self.previous_type {
            SerializerType::Map | SerializerType::Struct=> self.current.clear(),
            _ => (),
        }
        self.serialize_value(value);

        if self.should_pop{
            self.previous_keys.pop();
        }

        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {   
        if !self.current.ends_with('{') {
            self.current += ",";
        }
        self.is_key = true;
        key.serialize(&mut **self)?;
        self.is_key = false;

        self.current += ":";

        match self.previous_type {
            SerializerType::Map | SerializerType::Struct=> self.current.clear(),
            _ => (),
        }

        value.serialize(&mut **self);

        if self.should_pop{
            self.previous_keys.pop();
        }

        self.commit();
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    { Ok(())}

    fn end(self) -> Result<()> {Ok()}
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test<'a>{
            int: u32,
            mapping: std::collections::HashMap<&'a str, &'a str>,
        }

        let test = Test {
            int: 1,
            mapping: map!{"a" => "2", "b" => "three"},
        };
        
        let result = to_string(&test).unwrap();
        println!("{}", result);
        let expected1 = r#"{`int`:1,`mapping.a`:"2",`mapping.b`:"three"}}"#;
        let expected2 = r#"{`int`:1,`mapping.b`:"three",`mapping.a`:"2"}}"#;
        assert_eq!(true, (result == expected1) || (result == expected2));
    }
}

