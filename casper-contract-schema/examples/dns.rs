use casper_contract_schema::{ContractSchema, CustomType, Entrypoint, EnumVariant, Event, NamedType, Type, TypeName};
use casper_types::CLTyped;

type IPv4 = [u8; 4];
type IPv6 = [u8; 16];

#[allow(dead_code)]
enum IP {
    IPv4(IPv4), // single unnamed element,
    IPv4WithDescription(IPv4, String), // multiple unnamed elements,
    IPv6 { ip: IPv6 }, // single named element,
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
                members: vec![
                    NamedType::cl("ip", IPv6::cl_type()),
                ],
            },
            CustomType::Struct {
                name: TypeName::new("IP::IPv6WithDescription"),
                members: vec![
                    NamedType::cl("ip", IPv6::cl_type()),
                    NamedType::cl("description", String::cl_type()),
                ],
            },
            CustomType::Enum {
                name: TypeName::new("IP"),
                variants: vec![
                    EnumVariant {
                        name: String::from("IPv4"),
                        discriminant: 0,
                        ty: Type::System(IPv4::cl_type()),
                    },
                    EnumVariant {
                        name: String::from("IPv4WithDescription"),
                        discriminant: 1,
                        ty: Type::System(<(IPv4, String)>::cl_type()),
                    },
                    EnumVariant {
                        name: String::from("IPv6"),
                        discriminant: 2,
                        ty: Type::Custom(TypeName::new("IP::IPv6")),
                    },
                    EnumVariant {
                        name: String::from("IPv6WithDescription"),
                        discriminant: 3,
                        ty: Type::Custom(TypeName::new("IP::IPv6WithDescription")),
                    },
                ],
            },
            CustomType::Struct {
                name: TypeName::new("DNSRecord"),
                members: vec![
                    NamedType::cl("name", String::cl_type()),
                    NamedType::custom("ip", "IP"),
                ],
            }
        ],
        entry_points: vec![
            Entrypoint {
                name: String::from("add_record"),
                is_mutable: true,
                is_payable: true,
                args: vec![
                    NamedType::cl("name", String::cl_type()),
                    NamedType::custom("ip", "IP"),
                ],
                return_ty: Type::unit(),
                contract_context: true,
            },
            Entrypoint {
                name: String::from("remove_record"),
                is_mutable: true,
                is_payable: false,
                args: vec![
                    NamedType::cl("name", String::cl_type()),
                    NamedType::custom("ip", "IP"),
                ],
                return_ty: Type::unit(),
                contract_context: true,
            },
        ],
        events: vec![
            Event::new("event_RecordAdded", "DNSRecord"),
            Event::new("event_RecordRemoved", "DNSRecord")
        ],
    }
}

pub fn main() {
    let schema = enum_example_schema();
    let pretty_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", pretty_json);
}
