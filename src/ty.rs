use core::fmt;

use schemars::JsonSchema;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::{Serialize, SerializeStructVariant, Serializer},
    Deserialize, Deserializer,
};

/// CLType representation. It is slight extension of the original CLType enum.
/// Instead of `Any` variant, it uses `Custom` variant to represent custom
/// types, that can have name.
#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, JsonSchema, Debug, Hash)]
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

impl<'de> Deserialize<'de> for NamedCLType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NamedCLTypeVisitor;

        impl<'de> Visitor<'de> for NamedCLTypeVisitor {
            type Value = NamedCLType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a NamedCLType enum as string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match value {
                    "Bool" => NamedCLType::Bool,
                    "I32" => NamedCLType::I32,
                    "I64" => NamedCLType::I64,
                    "U8" => NamedCLType::U8,
                    "U32" => NamedCLType::U32,
                    "U64" => NamedCLType::U64,
                    "U128" => NamedCLType::U128,
                    "U256" => NamedCLType::U256,
                    "U512" => NamedCLType::U512,
                    "Unit" => NamedCLType::Unit,
                    "String" => NamedCLType::String,
                    "Key" => NamedCLType::Key,
                    "URef" => NamedCLType::URef,
                    "PublicKey" => NamedCLType::PublicKey,
                    other => NamedCLType::Custom(other.to_string()),
                })
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                // Expect a single key => value pair
                if let Some((key, value)) = access.next_entry::<String, serde_json::Value>()? {
                    match key.as_str() {
                        "Option" => {
                            let inner: NamedCLType =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::Option(Box::new(inner)))
                        }
                        "List" => {
                            let inner: NamedCLType =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::List(Box::new(inner)))
                        }
                        "ByteArray" => {
                            let len: u32 =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::ByteArray(len))
                        }
                        "Result" => {
                            let obj: serde_json::Value = value;
                            let ok: NamedCLType = serde_json::from_value(
                                obj.get("ok")
                                    .cloned()
                                    .ok_or_else(|| de::Error::missing_field("ok"))?,
                            )
                            .map_err(de::Error::custom)?;
                            let err: NamedCLType = serde_json::from_value(
                                obj.get("err")
                                    .cloned()
                                    .ok_or_else(|| de::Error::missing_field("err"))?,
                            )
                            .map_err(de::Error::custom)?;
                            Ok(NamedCLType::Result {
                                ok: Box::new(ok),
                                err: Box::new(err),
                            })
                        }
                        "Map" => {
                            let obj: serde_json::Value = value;
                            let key_ty: NamedCLType = serde_json::from_value(
                                obj.get("key")
                                    .cloned()
                                    .ok_or_else(|| de::Error::missing_field("key"))?,
                            )
                            .map_err(de::Error::custom)?;
                            let value_ty: NamedCLType = serde_json::from_value(
                                obj.get("value")
                                    .cloned()
                                    .ok_or_else(|| de::Error::missing_field("value"))?,
                            )
                            .map_err(de::Error::custom)?;
                            Ok(NamedCLType::Map {
                                key: Box::new(key_ty),
                                value: Box::new(value_ty),
                            })
                        }
                        "Tuple1" => {
                            let arr: [NamedCLType; 1] =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::Tuple1([Box::new(arr[0].clone())]))
                        }
                        "Tuple2" => {
                            let arr: [NamedCLType; 2] =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::Tuple2([
                                Box::new(arr[0].clone()),
                                Box::new(arr[1].clone()),
                            ]))
                        }
                        "Tuple3" => {
                            let arr: [NamedCLType; 3] =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::Tuple3([
                                Box::new(arr[0].clone()),
                                Box::new(arr[1].clone()),
                                Box::new(arr[2].clone()),
                            ]))
                        }
                        "Custom" => {
                            let s: String =
                                serde_json::from_value(value).map_err(de::Error::custom)?;
                            Ok(NamedCLType::Custom(s))
                        }
                        other => Err(de::Error::unknown_variant(
                            other,
                            &[
                                "Bool",
                                "String",
                                "U8",
                                "U32",
                                "Option",
                                "List",
                                "ByteArray",
                                "Result",
                                "Map",
                                "Tuple1",
                                "Tuple2",
                                "Tuple3",
                                "Custom",
                            ],
                        )),
                    }
                } else {
                    Err(de::Error::custom("expected a map with one key"))
                }
            }
        }

        deserializer.deserialize_any(NamedCLTypeVisitor)
    }
}
