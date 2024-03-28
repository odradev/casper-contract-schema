//! Data structure for representing a Casper Contract Schema.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod ty;
pub use ty::NamedCLType;

/// Contract definition.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ContractSchema {
    pub casper_contract_schema_version: u8,
    pub toolchain: String,
    pub authors: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub contract_name: String,
    pub contract_version: String,
    pub types: Vec<CustomType>,
    pub errors: Vec<UserError>,
    pub entry_points: Vec<Entrypoint>,
    pub events: Vec<Event>,
    pub call: Option<CallMethod>,
}

impl ContractSchema {
    /// Returns the JSON representation of the contract schema.
    pub fn as_json(&self) -> Option<String> {
        serde_json::to_string_pretty(self).ok()
    }
}

/// Entrypoint definition.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Entrypoint {
    pub name: String,
    pub description: Option<String>,
    pub is_mutable: bool,
    pub arguments: Vec<Argument>,
    pub return_ty: Type,
    pub is_contract_context: bool,
    pub access: Access,
}

/// Entrypoint's argument definition.
#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct Argument {
    pub name: String,
    pub description: Option<String>,
    pub ty: Type,
    pub optional: bool,
}

impl Argument {
    pub fn new(name: &str, description: &str, ty: NamedCLType) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(description),
            ty: ty.into(),
            optional: false,
        }
    }

    pub fn new_opt(name: &str, description: &str, ty: NamedCLType) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(description),
            ty: ty.into(),
            optional: true,
        }
    }
}

/// Access control definition.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Public,
    Groups(Vec<String>),
}

/// Struct member definition.
#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema, Debug)]
pub struct StructMember {
    pub name: String,
    pub description: Option<String>,
    pub ty: Type,
}

impl StructMember {
    /// Creates a new struct member.
    pub fn new(name: &str, description: &str, ty: NamedCLType) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(description),
            ty: ty.into(),
        }
    }
}

fn parse_description(description: &str) -> Option<String> {
    if description.is_empty() {
        None
    } else {
        Some(String::from(description))
    }
}

/// Type definition.
#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Type(pub NamedCLType);

impl From<NamedCLType> for Type {
    fn from(cl_type: NamedCLType) -> Self {
        Self(cl_type)
    }
}

/// Custom type name definition.
#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct TypeName(pub String);

impl From<&str> for TypeName {
    fn from(name: &str) -> Self {
        Self(String::from(name))
    }
}

impl From<String> for TypeName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl TypeName {
    /// Creates a new type name.
    pub fn new(name: &str) -> Self {
        Self(String::from(name))
    }
}

/// Custom type definition. It covers structs and enums.
#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CustomType {
    Struct {
        name: TypeName,
        description: Option<String>,
        members: Vec<StructMember>,
    },
    Enum {
        name: TypeName,
        description: Option<String>,
        variants: Vec<EnumVariant>,
    },
}

/// Enum variant definition.
#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema, Debug)]
pub struct EnumVariant {
    pub name: String,
    pub description: Option<String>,
    pub discriminant: u16,
    pub ty: Type,
}

/// Event definition.
#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema, Debug)]
pub struct Event {
    pub name: String,
    pub ty: TypeName,
}

impl Event {
    /// Creates a new event.
    pub fn new(name: &str, ty: &str) -> Self {
        Self {
            name: String::from(name),
            ty: ty.into(),
        }
    }
}

/// User error definition.
#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema, Debug)]
pub struct UserError {
    pub name: String,
    pub description: Option<String>,
    pub discriminant: u16,
}

impl UserError {
    /// Creates a new user error variant.
    pub fn new(name: &str, desc: &str, discriminant: u16) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(desc),
            discriminant,
        }
    }
}

/// Call method definition.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CallMethod {
    pub wasm_file_name: String,
    pub description: Option<String>,
    pub arguments: Vec<Argument>,
}

impl CallMethod {
    /// Creates a new call method definition.
    pub fn new(wasm_file_name: &str, description: &str, arguments: Vec<Argument>) -> Self {
        Self {
            wasm_file_name: String::from(wasm_file_name),
            description: parse_description(description),
            arguments,
        }
    }
}
