//! Storage layout of Odra modules.
//!
//! Every Odra module describes where each of its fields lives in the contract storage: the index
//! of the field, the kind of storage primitive and the types involved. The layout is a tree,
//! because a module can be composed of other modules. The root of the tree is the contract itself,
//! which is always a [StorageKind::Module].
//!
//! Available only when the `odra` feature is enabled.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{NamedCLType, Type};

/// A single field of a module together with its index in the module.
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct StorageField {
    /// The field name as written in the module struct.
    pub name: String,
    /// The index of the field in the module (1-based, `env` excluded).
    pub index: u8,
    /// What is stored under the field.
    #[serde(flatten)]
    pub kind: StorageKind,
}

impl StorageField {
    /// Creates a new field description.
    pub fn new(name: &str, index: u8, kind: StorageKind) -> Self {
        Self {
            name: String::from(name),
            index,
            kind,
        }
    }
}

/// How a dictionary item key is derived from the key value.
#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KeyEncoding {
    /// The key is a string used as-is.
    Utf8,
    /// The serialized key is base64-encoded.
    Base64,
    /// The serialized key is hashed with `blake2b` and hex-encoded.
    HexHash,
}

/// Describes what is stored under a module element.
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StorageKind {
    /// A single value (`Var<T>`, `External<T>`).
    Value {
        /// The type of the value.
        ty: Type,
    },
    /// A key-value store (`Mapping<K, V>`).
    Mapping {
        /// The type of the key.
        key: Type,
        /// What is stored under a key - a value or a whole module.
        value: Box<StorageKind>,
    },
    /// An indexed collection (`List<T>`).
    List {
        /// The type of the items.
        item: Type,
    },
    /// A counter (`Sequence<T>`).
    Sequence {
        /// The type of the counter.
        ty: Type,
    },
    /// A composition of other modules (`SubModule<M>` or the contract itself).
    Module {
        /// The fields of the module.
        fields: Vec<StorageField>,
    },
    /// A value stored under a Casper named key.
    NamedKey {
        /// The name of the named key. Serialized as `named_key`, so it does not
        /// collide with [StorageField::name] when flattened.
        #[serde(rename = "named_key")]
        name: String,
        /// The type of the value.
        ty: Type,
    },
    /// A value stored in a Casper dictionary.
    Dictionary {
        /// The name of the dictionary. Serialized as `dictionary`, so it does not
        /// collide with [StorageField::name] when flattened.
        #[serde(rename = "dictionary")]
        name: String,
        /// The type of the key.
        key: Type,
        /// How the dictionary item key is derived from the key value.
        key_encoding: KeyEncoding,
        /// The type of the value.
        value: Type,
    },
}

impl StorageKind {
    /// A single value of the given type.
    pub fn value(ty: NamedCLType) -> Self {
        Self::Value { ty: ty.into() }
    }

    /// A key-value store with keys of the given type.
    pub fn mapping(key: NamedCLType, value: StorageKind) -> Self {
        Self::Mapping {
            key: key.into(),
            value: Box::new(value),
        }
    }

    /// An indexed collection of items of the given type.
    pub fn list(item: NamedCLType) -> Self {
        Self::List { item: item.into() }
    }

    /// A counter of the given type.
    pub fn sequence(ty: NamedCLType) -> Self {
        Self::Sequence { ty: ty.into() }
    }

    /// A module composed of the given fields.
    pub fn module(fields: Vec<StorageField>) -> Self {
        Self::Module { fields }
    }

    /// A value of the given type stored under a named key.
    pub fn named_key(name: &str, ty: NamedCLType) -> Self {
        Self::NamedKey {
            name: String::from(name),
            ty: ty.into(),
        }
    }

    /// A value stored in a dictionary under a key of the given type.
    pub fn dictionary(
        name: &str,
        key: NamedCLType,
        key_encoding: KeyEncoding,
        value: NamedCLType,
    ) -> Self {
        Self::Dictionary {
            name: String::from(name),
            key: key.into(),
            key_encoding,
            value: value.into(),
        }
    }

    /// The fields of a module, empty for any other kind.
    pub fn fields(&self) -> &[StorageField] {
        match self {
            Self::Module { fields } => fields,
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token() -> StorageKind {
        StorageKind::module(vec![
            StorageField::new("name", 1, StorageKind::value(NamedCLType::String)),
            StorageField::new(
                "balances",
                2,
                StorageKind::mapping(NamedCLType::Key, StorageKind::value(NamedCLType::U256)),
            ),
            StorageField::new("holders", 3, StorageKind::list(NamedCLType::Key)),
            StorageField::new("ids", 4, StorageKind::sequence(NamedCLType::U32)),
        ])
    }

    fn loans() -> StorageKind {
        StorageKind::module(vec![
            StorageField::new("lenders", 1, token()),
            StorageField::new("borrowers", 2, token()),
            StorageField::new(
                "decimals",
                3,
                StorageKind::named_key("decimals", NamedCLType::U8),
            ),
            StorageField::new(
                "allowances",
                4,
                StorageKind::dictionary(
                    "allowances",
                    NamedCLType::String,
                    KeyEncoding::Utf8,
                    NamedCLType::U256,
                ),
            ),
        ])
    }

    #[test]
    fn serializes_layout() {
        let json = serde_json::to_value(loans()).unwrap();
        assert_eq!(json["kind"], "module");
        assert_eq!(json["fields"][0]["name"], "lenders");
        assert_eq!(json["fields"][0]["index"], 1);
        assert_eq!(json["fields"][0]["fields"][1]["kind"], "mapping");
        assert_eq!(json["fields"][0]["fields"][1]["key"], "Key");
        assert_eq!(json["fields"][0]["fields"][1]["value"]["kind"], "value");
        assert_eq!(json["fields"][2]["name"], "decimals");
        assert_eq!(json["fields"][2]["named_key"], "decimals");
        assert_eq!(json["fields"][3]["name"], "allowances");
        assert_eq!(json["fields"][3]["dictionary"], "allowances");
        assert_eq!(json["fields"][3]["key_encoding"], "utf8");
    }

    #[test]
    fn roundtrips_layout() {
        let layout = loans();
        let json = serde_json::to_string(&layout).unwrap();
        let decoded: StorageKind = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, layout);
    }

    #[test]
    fn generates_json_schema() {
        let schema = schemars::schema_for!(StorageKind);
        let json = serde_json::to_value(&schema).unwrap();
        assert!(json["oneOf"].is_array() || json["anyOf"].is_array());
    }
}
