use casper_contract_schema::*;
use casper_types::{CLTyped, Key, U256};

pub fn example_erc20_schema() -> ContractSchema {
    ContractSchema {
        casper_contract_schema_version: 1,
        toolchain: String::from("rustc 1.73.0 (cc66ad468 2023-10-03)"),
        name: String::from("Erc20"),
        entry_points: vec![
            Entrypoint {
                name: String::from("transfer"),
                is_mutable: true,
                args: vec![
                    NamedType::cl("recipient", Key::cl_type()),
                    NamedType::cl("amount", U256::cl_type()),
                ],
                return_ty: Type::unit(),
                contract_context: true,
            },
            Entrypoint {
                name: String::from("transfer_from"),
                is_mutable: true,
                args: vec![
                    NamedType::cl("owner", Key::cl_type()),
                    NamedType::cl("recipient", Key::cl_type()),
                    NamedType::cl("amount", U256::cl_type()),
                ],
                return_ty: Type::unit(),
                contract_context: true,
            },
            Entrypoint {
                name: String::from("approve"),
                is_mutable: true,
                args: vec![
                    NamedType::cl("spender", Key::cl_type()),
                    NamedType::cl("amount", U256::cl_type()),
                ],
                return_ty: Type::unit(),
                contract_context: true,
            },
            Entrypoint {
                name: String::from("allowance"),
                is_mutable: false,
                args: vec![
                    NamedType::cl("owner", Key::cl_type()),
                    NamedType::cl("spender", Key::cl_type()),
                ],
                return_ty: Type::System(U256::cl_type()),
                contract_context: true,
            },
            Entrypoint {
                name: String::from("balance_of"),
                is_mutable: false,
                args: vec![NamedType::cl("owner", Key::cl_type())],
                return_ty: Type::System(U256::cl_type()),
                contract_context: true,
            },
            Entrypoint {
                name: String::from("total_supply"),
                is_mutable: false,
                args: vec![],
                return_ty: Type::System(U256::cl_type()),
                contract_context: true,
            },
        ],
        types: vec![
            UserDefinedType {
                name: UserDefinedTypeName::new("Transfer"),
                members: vec![
                    NamedType::cl("from", Option::<Key>::cl_type()),
                    NamedType::cl("to", Option::<Key>::cl_type()),
                    NamedType::cl("value", U256::cl_type()),
                ],
            },
            UserDefinedType {
                name: UserDefinedTypeName::new("Approval"),
                members: vec![
                    NamedType::cl("owner", Option::<Key>::cl_type()),
                    NamedType::cl("spender", Option::<Key>::cl_type()),
                    NamedType::cl("value", U256::cl_type()),
                ],
            },
        ],
        events: vec![Event::custom("Transfer"), Event::custom("Approval")],
    }
}

pub fn main() {
    let schema = example_erc20_schema();
    let pretty_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", pretty_json);
}
