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
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Entrypoint {
    pub name: String,
    pub is_mutable: bool,
    pub is_payable: bool,
    pub args: Vec<NamedType>,
    pub return_ty: Type,
    pub contract_context: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NamedType {
    pub name: String,
    pub ty: Type,
}

impl NamedType {
    pub fn cl(name: &str, ty: CLType) -> Self {
        Self {
            name: String::from(name),
            ty: Type::System(ty),
        }
    }

    pub fn custom(name: &str, ty: &str) -> Self {
        Self {
            name: String::from(name),
            ty: Type::Custom(TypeName::new(ty)),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    Struct { name: TypeName, members: Vec<NamedType> },
    Enum { name: TypeName, variants: Vec<EnumVariant> },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EnumVariant {
    pub name: String,
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
