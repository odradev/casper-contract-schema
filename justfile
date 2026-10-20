update-schema:
    cargo run --example erc20 > examples/erc20_schema.json
    cargo run --example dns > examples/dns_schema.json
    cargo run --example erc20 --features storage > examples/erc20_schema_with_storage.json
    cargo run --example dns --features storage > examples/dns_schema_with_storage.json