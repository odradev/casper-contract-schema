use casper_contract_schema::{
    Access, Argument, ContractSchema, CustomType, Entrypoint, EnumVariant, Event, NamedCLType,
    StructMember, TypeName,
};

type IPv4 = [u8; 4];
type IPv6 = [u8; 16];

#[allow(dead_code)]
enum IP {
    Unknown,                                               // no data,
    IPv4(IPv4),                                            // single unnamed element,
    IPv4WithDescription(IPv4, String),                     // multiple unnamed elements,
    IPv6 { ip: IPv6 },                                     // single named element,
    IPv6WithDescription { ip: IPv6, description: String }, // multiple named elements,
}

fn enum_example_schema() -> ContractSchema {
    ContractSchema {
        casper_contract_schema_version: 1,
        toolchain: String::from("rustc 1.73.0 (cc66ad468 2023-10-03)"),
        contract_name: String::from("DNS"),
        contract_version: String::from("0.1.3"),
        authors: vec![],
        repository: None,
        homepage: None,
        errors: vec![],
        types: vec![
            CustomType::Struct {
                name: TypeName::new("IP::IPv6"),
                description: None,
                members: vec![StructMember::new("ip", "", NamedCLType::ByteArray(16))],
            },
            CustomType::Struct {
                name: TypeName::new("IP::IPv6WithDescription"),
                description: None,
                members: vec![
                    StructMember::new("ip", "", NamedCLType::ByteArray(16)),
                    StructMember::new("description", "", NamedCLType::String),
                ],
            },
            CustomType::Enum {
                name: TypeName::new("IP"),
                description: Some(String::from("IP address")),
                variants: vec![
                    EnumVariant {
                        name: String::from("Unknown"),
                        description: None,
                        discriminant: 0,
                        ty: NamedCLType::Unit.into(),
                    },
                    EnumVariant {
                        name: String::from("IPv4"),
                        description: Some(String::from("IPv4 address")),
                        discriminant: 1,
                        ty: NamedCLType::ByteArray(4).into(),
                    },
                    EnumVariant {
                        name: String::from("IPv4WithDescription"),
                        description: Some(String::from("IPv4 address with description")),
                        discriminant: 2,
                        ty: NamedCLType::Tuple2([
                            Box::new(NamedCLType::ByteArray(4)),
                            Box::new(NamedCLType::String),
                        ])
                        .into(),
                    },
                    EnumVariant {
                        name: String::from("IPv6"),
                        description: Some(String::from("IPv6 address")),
                        discriminant: 3,
                        ty: NamedCLType::Custom("IP::IPv6".to_owned()).into(),
                    },
                    EnumVariant {
                        name: String::from("IPv6WithDescription"),
                        description: Some(String::from("IPv6 address with description")),
                        discriminant: 4,
                        ty: NamedCLType::Custom("IP::IPv6WithDescription".to_owned()).into(),
                    },
                ],
            },
            CustomType::Struct {
                name: TypeName::new("DNSRecord"),
                description: Some(String::from("DNS record")),
                members: vec![
                    StructMember::new("name", "Domain name", NamedCLType::String),
                    StructMember::new("ip", "", NamedCLType::Custom("IP".to_owned())),
                ],
            },
        ],
        entry_points: vec![
            Entrypoint {
                name: String::from("add_record"),
                description: None,
                is_mutable: true,
                arguments: vec![
                    Argument::new("name", "", NamedCLType::String),
                    Argument::new("ip", "", NamedCLType::Custom("IP".to_owned())),
                ],
                return_ty: NamedCLType::Unit.into(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("remove_record"),
                description: Some(String::from("Remove a DNS record")),
                is_mutable: true,
                arguments: vec![
                    Argument::new("name", "", NamedCLType::String),
                    Argument::new("ip", "", NamedCLType::Custom("IP".to_owned())),
                ],
                return_ty: NamedCLType::Unit.into(),
                is_contract_context: true,
                access: Access::Groups(vec![String::from("admin"), String::from("moderator")]),
            },
        ],
        events: vec![
            Event::new("event_RecordAdded", "DNSRecord"),
            Event::new("event_RecordRemoved", "DNSRecord"),
        ],
        call: None,
    }
}

pub fn main() {
    let schema = enum_example_schema();
    let pretty_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", pretty_json);
}
