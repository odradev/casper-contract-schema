use casper_types::CLType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ContractSchema {
    pub casper_contract_schema_version: u8,
    pub toolchain: String,
    pub contract_name: String,
    pub contract_version: String,
    pub types: Vec<CustomType>,
    pub entry_points: Vec<Entrypoint>,
    pub events: Vec<Event>,
    pub call: Option<CallMethod>,
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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Argument {
    pub name: String,
    pub description: Option<String>,
    pub ty: Type,
    pub optional: bool,
}

impl Argument {
    pub fn cl(name: &str, description: &str, ty: CLType) -> Self {
        Self::new(name, description, Type::System(ty), false)
    }

    pub fn opt_cl(name: &str, description: &str, ty: CLType) -> Self {
        Self::new(name, description, Type::System(ty), true)
    }

    pub fn custom(name: &str, description: &str, ty: &str) -> Self {
        Self::new(name, description, Type::Custom(TypeName::new(ty)), false)
    }

    pub fn opt_custom(name: &str, description: &str, ty: &str) -> Self {
        Self::new(name, description, Type::Custom(TypeName::new(ty)), true)
    }

    fn new(name: &str, description: &str, ty: Type, optional: bool) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(description),
            ty,
            optional,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Public,
    Groups(Vec<String>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StructMember {
    pub name: String,
    pub description: Option<String>,
    pub ty: Type
}

impl StructMember {
    pub fn cl(name: &str, description: &str, ty: CLType) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(description),
            ty: Type::System(ty),
        }
    }

    pub fn custom(name: &str, description: &str, ty: &str) -> Self {
        Self {
            name: String::from(name),
            description: parse_description(description),
            ty: Type::Custom(TypeName::new(ty)),
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Type {
    System(CLType),
    Custom(TypeName),
}

impl Type {
    pub fn unit() -> Self {
        Self::System(CLType::Unit)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct TypeName(pub String);

impl TypeName {
    pub fn new(name: &str) -> Self {
        Self(String::from(name))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CustomType {
    Struct { name: TypeName, description: Option<String>, members: Vec<StructMember> },
    Enum { name: TypeName, description: Option<String>, variants: Vec<EnumVariant> },
}

#[derive(Serialize, Deserialize, JsonSchema)]
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
            ty: TypeName(String::from(ty)),
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