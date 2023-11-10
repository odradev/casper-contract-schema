# Casper Contract Schema

Repository that includes The Casper Contract Schema.
It sepcifies how to comunicate with a contract.

## Usage

See [json-schema.json](./json-schema.json) for schema definition.

See [erc20_schema.json](./erc20_schema.json) for an example usage.

To update the schema after code changes, run:
```bash
$ just update-schema
```

## Notes
- We don't include the contract groups information.

## Tasks
- [ ] Should we include serialization specification for structs and enums?
- [ ] Should we include type name in front of the type serialized version?
- [ ] Should we use Any, or we should always expect custom type?

## Future work
- IDE intergration
- Codegen for JS, Rust, C#
