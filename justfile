update-schema:
    cd casper-contract-schema && cargo run --example erc20 > examples/erc20_schema.json
    cd casper-contract-schema && cargo run --example dns > examples/dns_schema.json