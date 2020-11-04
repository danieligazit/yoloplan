#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SerializerType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char,
    Str,
    Bytes,
    None,
    Some,
    Unit,
    UnitStruct,
    UnitVariant,
    NewTypeStruct,
    NewTypeVariant,
    Seq,
    Tuple,
    TupleStruct,
    TupleVariant,
    Map,
    Struct,
    StructVariant,
} 