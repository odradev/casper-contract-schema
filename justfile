update-schema:
    cd casper-contract-schema && cargo run --example erc20 > ../erc20_schema.json
    cd casper-contract-schema && cargo run --bin json-schema > ../json-schema.json