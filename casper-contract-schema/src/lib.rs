use casper_types::CLType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod ty;
pub use ty::NamedCLType;

#[derive(Serialize, Deserialize, JsonSchema)]
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
    pub fn as_json(&self) -> Option<String> {
        serde_json::to_string_pretty(self).ok()
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Entrypoint {
    pub name: String,
    pub description: Option<String>,
    pub is_mutable: bool,
    pub arguments: Vec<Argument>,
    pub return_ty: Type,
    pub is_contract_context: bool,
    pub access: Access,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Public,
    Groups(Vec<String>),
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StructMember {
    pub name: String,
    pub description: Option<String>,
    pub ty: Type,
}

impl StructMember {
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

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CLType3 {
    /// `bool` primitive.
    Bool,
    /// `i32` primitive.
    I32,
    /// `i64` primitive.
    I64,
    /// `u8` primitive.
    U8,
    /// `u32` primitive.
    U32,
    /// `u64` primitive.
    U64,
    /// `Option` of a `CLType`.
    Option(Box<CLType>),
    /// Variable-length list of a single `CLType` (comparable to a `Vec`).
    ByteArray(u32),
    /// `Result` with `Ok` and `Err` variants of `CLType`s.
    Result {
        ok: Box<CLType>,
        err: Box<CLType>,
    },
    Any,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type(pub NamedCLType);

impl From<NamedCLType> for Type {
    fn from(cl_type: NamedCLType) -> Self {
        Self(cl_type)
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Serialize, Deserialize, JsonSchema)]
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
    pub fn new(name: &str) -> Self {
        Self(String::from(name))
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema)]
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

#[derive(PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EnumVariant {
    pub name: String,
    pub description: Option<String>,
    pub discriminant: u8,
    pub ty: Type,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Event {
    pub name: String,
    pub ty: TypeName,
}

impl Event {
    pub fn new(name: &str, ty: &str) -> Self {
        Self {
            name: String::from(name),
            ty: ty.into(),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserError {
    pub name: String,
    pub description: Option<String>,
    pub discriminant: u8,
}

impl UserError {
    pub fn new(name: &str, desc: &str, discriminant: u8) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(desc),
            discriminant,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CallMethod {
    pub wasm_file_name: String,
    pub description: Option<String>,
    pub arguments: Vec<Argument>,
}

impl CallMethod {
    pub fn new(wasm_file_name: &str, description: &str, arguments: Vec<Argument>) -> Self {
        Self {
            wasm_file_name: String::from(wasm_file_name),
            description: parse_description(description),
            arguments,
        }
    }
}
