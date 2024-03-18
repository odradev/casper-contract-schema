use schemars::JsonSchema;
use serde::{
    ser::{Serialize, SerializeStructVariant, Serializer},
    Deserialize,
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Deserialize, JsonSchema)]
pub enum NamedCLType {
    Bool,
    I32,
    I64,
    U8,
    U32,
    U64,
    U128,
    U256,
    U512,
    Unit,
    String,
    Key,
    URef,
    PublicKey,
    Option(Box<NamedCLType>),
    List(Box<NamedCLType>),
    ByteArray(u32),
    Result {
        ok: Box<NamedCLType>,
        err: Box<NamedCLType>,
    },
    Map {
        key: Box<NamedCLType>,
        value: Box<NamedCLType>,
    },
    Tuple1([Box<NamedCLType>; 1]),
    Tuple2([Box<NamedCLType>; 2]),
    Tuple3([Box<NamedCLType>; 3]),
    Custom(String),
}

impl Serialize for NamedCLType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NamedCLType::Bool => serializer.serialize_unit_variant("NamedCLType", 0, "Bool"),
            NamedCLType::I32 => serializer.serialize_unit_variant("NamedCLType", 1, "I32"),
            NamedCLType::I64 => serializer.serialize_unit_variant("NamedCLType", 2, "I64"),
            NamedCLType::U8 => serializer.serialize_unit_variant("NamedCLType", 3, "U8"),
            NamedCLType::U32 => serializer.serialize_unit_variant("NamedCLType", 4, "U32"),
            NamedCLType::U64 => serializer.serialize_unit_variant("NamedCLType", 5, "U64"),
            NamedCLType::U128 => serializer.serialize_unit_variant("NamedCLType", 6, "U128"),
            NamedCLType::U256 => serializer.serialize_unit_variant("NamedCLType", 7, "U256"),
            NamedCLType::U512 => serializer.serialize_unit_variant("NamedCLType", 8, "U512"),
            NamedCLType::Unit => serializer.serialize_unit_variant("NamedCLType", 9, "Unit"),
            NamedCLType::String => serializer.serialize_unit_variant("NamedCLType", 10, "String"),
            NamedCLType::Key => serializer.serialize_unit_variant("NamedCLType", 11, "Key"),
            NamedCLType::URef => serializer.serialize_unit_variant("NamedCLType", 12, "URef"),
            NamedCLType::PublicKey => {
                serializer.serialize_unit_variant("NamedCLType", 13, "PublicKey")
            }
            NamedCLType::Option(f) => {
                serializer.serialize_newtype_variant("NamedCLType", 14, "Option", f)
            }
            NamedCLType::List(f) => {
                serializer.serialize_newtype_variant("NamedCLType", 15, "List", f)
            }
            NamedCLType::ByteArray(f) => {
                serializer.serialize_newtype_variant("NamedCLType", 16, "ByteArray", f)
            }
            NamedCLType::Result { ok, err } => {
                let mut s =
                    serializer.serialize_struct_variant("NamedCLType", 17, "Result", 0 + 1 + 1)?;
                s.serialize_field("ok", ok)?;
                s.serialize_field("err", err)?;
                s.end()
            }
            NamedCLType::Map { key, value } => {
                let mut s =
                    serializer.serialize_struct_variant("NamedCLType", 18, "Map", 0 + 1 + 1)?;
                s.serialize_field("key", key)?;
                s.serialize_field("value", value)?;
                s.end()
            }
            NamedCLType::Tuple1(f) => {
                serializer.serialize_newtype_variant("NamedCLType", 19, "Tuple1", f)
            }
            NamedCLType::Tuple2(f) => {
                serializer.serialize_newtype_variant("NamedCLType", 20, "Tuple2", f)
            }
            NamedCLType::Tuple3(f) => {
                serializer.serialize_newtype_variant("NamedCLType", 21, "Tuple3", f)
            }
            NamedCLType::Custom(name) => serializer.serialize_str(name),
        }
    }
}
