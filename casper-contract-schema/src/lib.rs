use casper_types::CLType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ContractSchema {
    pub casper_contract_schema_version: u8,
    pub toolchain: String,
    pub name: String,
    pub entry_points: Vec<Entrypoint>,
    pub types: Vec<UserDefinedType>,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Entrypoint {
    pub name: String,
    pub is_mutable: bool,
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
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Type {
    System(CLType),
    UserDefined(UserDefinedTypeName),
}

impl Type {
    pub fn unit() -> Self {
        Self::System(CLType::Unit)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserDefinedTypeName(pub String);

impl UserDefinedTypeName {
    pub fn new(name: &str) -> Self {
        Self(String::from(name))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserDefinedType {
    pub name: UserDefinedTypeName,
    pub members: Vec<NamedType>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Event {
    pub name: String,
    pub ty: Type,
}

impl Event {
    pub fn custom(name: &str) -> Self {
        Self {
            name: String::from(name),
            ty: Type::UserDefined(UserDefinedTypeName(String::from(name))),
        }
    }
}
