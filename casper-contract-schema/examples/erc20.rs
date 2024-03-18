use casper_contract_schema::*;

pub fn example_erc20_schema() -> ContractSchema {
    ContractSchema {
        casper_contract_schema_version: 1,
        toolchain: String::from("rustc 1.73.0 (cc66ad468 2023-10-03)"),
        contract_name: String::from("Erc20"),
        contract_version: String::from("0.1.0"),
        authors: vec![String::from("John Doe <john@doe.com>")],
        repository: Some(String::from(
            "https://github.com/casper-ecosystem/casper-contract-schema",
        )),
        homepage: Some(String::from("https://john.doe.com")),
        errors: vec![
            UserError {
                name: String::from("InsufficientFunds"),
                description: Some(String::from("Insufficient funds")),
                discriminant: 100u8,
            },
            UserError {
                name: String::from("InsufficientAllowance"),
                description: Some(String::from("Insufficient allowance")),
                discriminant: 101u8,
            },
        ],
        types: vec![
            CustomType::Struct {
                name: TypeName::new("Transfer"),
                description: Some(String::from("Transfer event")),
                members: vec![
                    StructMember::new(
                        "from",
                        "Sender of tokens.",
                        NamedCLType::Option(Box::new(NamedCLType::Key)),
                    ),
                    StructMember::new(
                        "to",
                        "Recipient of tokens.",
                        NamedCLType::Option(Box::new(NamedCLType::Key)),
                    ),
                    StructMember::new("value", "Transfered amount.", NamedCLType::U256),
                ],
            },
            CustomType::Struct {
                name: TypeName::new("Approval"),
                description: Some(String::from("Approval event")),
                members: vec![
                    StructMember::new("owner", "", NamedCLType::Option(Box::new(NamedCLType::Key))),
                    StructMember::new(
                        "spender",
                        "",
                        NamedCLType::Option(Box::new(NamedCLType::Key)),
                    ),
                    StructMember::new("value", "", NamedCLType::U256),
                ],
            },
        ],
        entry_points: vec![
            Entrypoint {
                name: String::from("transfer"),
                description: Some(String::from("Transfer tokens to another account")),
                is_mutable: true,
                arguments: vec![
                    Argument::new("recipient", "", NamedCLType::Key),
                    Argument::new("amount", "", NamedCLType::U256),
                ],
                return_ty: NamedCLType::Unit.into(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("transfer_from"),
                description: Some(String::from("Transfer tokens from one account to another")),
                is_mutable: true,
                arguments: vec![
                    Argument::new("owner", "", NamedCLType::Key),
                    Argument::new("recipient", "", NamedCLType::Key),
                    Argument::new("amount", "", NamedCLType::U256),
                ],
                return_ty: NamedCLType::Unit.into(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("approve"),
                description: Some(String::from(
                    "Approve spender to use tokens on behalf of the owner",
                )),
                is_mutable: true,
                arguments: vec![
                    Argument::new("spender", "", NamedCLType::Key),
                    Argument::new("amount", "", NamedCLType::U256),
                ],
                return_ty: NamedCLType::Unit.into(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("allowance"),
                description: Some(String::from(
                    "Check the amount of tokens that an owner allowed to a spender",
                )),
                is_mutable: false,
                arguments: vec![
                    Argument::new("owner", "", NamedCLType::Key),
                    Argument::new("spender", "", NamedCLType::Key),
                ],
                return_ty: NamedCLType::U256.into(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("balance_of"),
                description: Some(String::from(
                    "Check the amount of tokens owned by an account",
                )),
                is_mutable: false,
                arguments: vec![Argument::new("owner", "", NamedCLType::Key)],
                return_ty: NamedCLType::U256.into(),
                is_contract_context: true,
                access: Access::Public,
            },
            Entrypoint {
                name: String::from("total_supply"),
                description: Some(String::from("Check the total supply of tokens")),
                is_mutable: false,
                arguments: vec![],
                return_ty: NamedCLType::U256.into(),
                is_contract_context: true,
                access: Access::Public,
            },
        ],
        events: vec![
            Event::new("event_Transfer", "Transfer"),
            Event::new("event_Approval", "Approval"),
        ],
        call: Some(CallMethod::new(
            "erc20.wasm",
            "Fungible token",
            vec![
                Argument::new("name", "Name of the token", NamedCLType::String),
                Argument::new("symbol", "Symbol of the token", NamedCLType::String),
                Argument::new("decimals", "Number of decimals", NamedCLType::U8),
                Argument::new(
                    "initial_supply",
                    "Initial supply of tokens",
                    NamedCLType::U256,
                ),
            ],
        )),
    }
}

pub fn main() {
    let schema = example_erc20_schema();
    let pretty_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", pretty_json);
}
