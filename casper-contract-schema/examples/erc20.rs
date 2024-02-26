use casper_contract_schema::*;
use casper_types::{CLTyped, Key, U256};

pub fn example_erc20_schema() -> ContractSchema {
    ContractSchema {
        casper_contract_schema_version: 1,
        toolchain: String::from("rustc 1.73.0 (cc66ad468 2023-10-03)"),
        contract_name: String::from("Erc20"),
        contract_version: String::from("0.1.0"),
        types: vec![
            CustomType::Struct {
                name: TypeName::new("Transfer"),
                description: Some(String::from("Transfer event")),
                members: vec![
                    StructMember::cl("from", "Sender of tokens.", Option::<Key>::cl_type()),
                    StructMember::cl("to", "Recipient of tokens.", Option::<Key>::cl_type()),
                    StructMember::cl("value", "Transfered amount.", U256::cl_type()),
                ],
            },
            CustomType::Struct {
                name: TypeName::new("Approval"),
                description: Some(String::from("Approval event")),
                members: vec![
                    StructMember::cl("owner", "", Option::<Key>::cl_type()),
                    StructMember::cl("spender", "", Option::<Key>::cl_type()),
                    StructMember::cl("value", "", U256::cl_type()),
                ],
            },
        ],
        entry_points: vec![
            Entrypoint {
                name: String::from("transfer"),
                description: Some(String::from("Transfer tokens to another account")),
                is_mutable: true,
                arguments: vec![
                    Argument::cl("recipient", "", Key::cl_type()),
                    Argument::cl("amount", "", U256::cl_type()),
                ],
                return_ty: Type::unit(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("transfer_from"),
                description: Some(String::from("Transfer tokens from one account to another")),
                is_mutable: true,
                arguments: vec![
                    Argument::cl("owner", "", Key::cl_type()),
                    Argument::cl("recipient", "", Key::cl_type()),
                    Argument::cl("amount", "", U256::cl_type()),
                ],
                return_ty: Type::unit(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("approve"),
                description: Some(String::from("Approve spender to use tokens on behalf of the owner")),
                is_mutable: true,
                arguments: vec![
                    Argument::cl("spender", "", Key::cl_type()),
                    Argument::cl("amount", "", U256::cl_type()),
                ],
                return_ty: Type::unit(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("allowance"),
                description: Some(String::from("Check the amount of tokens that an owner allowed to a spender")),
                is_mutable: false,
                arguments: vec![
                    Argument::cl("owner", "", Key::cl_type()),
                    Argument::cl("spender", "", Key::cl_type()),
                ],
                return_ty: Type::System(U256::cl_type()),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("balance_of"),
                description: Some(String::from("Check the amount of tokens owned by an account")),
                is_mutable: false,
                arguments: vec![Argument::cl("owner", "", Key::cl_type())],
                return_ty: Type::System(U256::cl_type()),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("total_supply"),
                description: Some(String::from("Check the total supply of tokens")),
                is_mutable: false,
                arguments: vec![],
                return_ty: Type::System(U256::cl_type()),
                is_contract_context: true,
                access: Access::Public,
            },
        ],
        events: vec![
            Event::new("event_Transfer", "Transfer"),
            Event::new("event_Approval", "Approval")
        ],
        call: Some(CallMethod::new("erc20.wasm", "Fungable token", vec![
            Argument::cl("name", "Name of the token", String::cl_type()),
            Argument::cl("symbol", "Symbol of the token", String::cl_type()),
            Argument::cl("decimals", "Number of decimals", u8::cl_type()),
            Argument::cl("initial_supply", "Initial supply of tokens", U256::cl_type()),
        ]))
    }
}

pub fn main() {
    let schema = example_erc20_schema();
    let pretty_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", pretty_json);
}
