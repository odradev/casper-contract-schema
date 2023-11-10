use schemars::schema_for;

pub fn main() {
    let schema = schema_for!(casper_contract_schema::ContractSchema);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
