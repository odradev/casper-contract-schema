use casper_contract_schema::{
    Access, Argument, ContractSchema, CustomType, Entrypoint, EnumVariant, Event, StructMember,
    Type, TypeName,
};
use casper_types::CLTyped;

type IPv4 = [u8; 4];
type IPv6 = [u8; 16];

#[allow(dead_code)]
enum IP {
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
        types: vec![
            CustomType::Struct {
                name: TypeName::new("IP::IPv6"),
                description: None,
                members: vec![StructMember::cl("ip", "", IPv6::cl_type())],
            },
            CustomType::Struct {
                name: TypeName::new("IP::IPv6WithDescription"),
                description: None,
                members: vec![
                    StructMember::cl("ip", "", IPv6::cl_type()),
                    StructMember::cl("description", "", String::cl_type()),
                ],
            },
            CustomType::Enum {
                name: TypeName::new("IP"),
                description: Some(String::from("IP address")),
                variants: vec![
                    EnumVariant {
                        name: String::from("IPv4"),
                        description: Some(String::from("IPv4 address")),
                        discriminant: 0,
                        ty: Type::System(IPv4::cl_type()),
                    },
                    EnumVariant {
                        name: String::from("IPv4WithDescription"),
                        description: Some(String::from("IPv4 address with description")),
                        discriminant: 1,
                        ty: Type::System(<(IPv4, String)>::cl_type()),
                    },
                    EnumVariant {
                        name: String::from("IPv6"),
                        description: Some(String::from("IPv6 address")),
                        discriminant: 2,
                        ty: Type::Custom(TypeName::new("IP::IPv6")),
                    },
                    EnumVariant {
                        name: String::from("IPv6WithDescription"),
                        description: Some(String::from("IPv6 address with description")),
                        discriminant: 3,
                        ty: Type::Custom(TypeName::new("IP::IPv6WithDescription")),
                    },
                ],
            },
            CustomType::Struct {
                name: TypeName::new("DNSRecord"),
                description: Some(String::from("DNS record")),
                members: vec![
                    StructMember::cl("name", "Domain name", String::cl_type()),
                    StructMember::custom("ip", "", "IP"),
                ],
            },
        ],
        entry_points: vec![
            Entrypoint {
                name: String::from("add_record"),
                description: None,
                is_mutable: true,
                arguments: vec![
                    Argument::cl("name", "", String::cl_type()),
                    Argument::custom("ip", "", "IP"),
                ],
                return_ty: Type::unit(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("remove_record"),
                description: Some(String::from("Remove a DNS record")),
                is_mutable: true,
                arguments: vec![
                    Argument::cl("name", "", String::cl_type()),
                    Argument::custom("ip", "", "IP"),
                ],
                return_ty: Type::unit(),
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
