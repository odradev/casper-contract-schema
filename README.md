# Casper Contract Schema

Repository that includes The Casper Contract Schema.
It sepcifies how to communicate with a contract.

## Examples
See [erc20_schema.json](./casper-contract-schema/examples/erc20_schema.json) and
[dns_schema.json](./casper-contract-schema/examples/dns_schema.json) for example
usages.

To update the schema after code changes, run:
```bash
$ just update-schema
```

## Schema Format

Following is the rough description of the schema format.

### Root
Root of schema is an object with the following properties:
- `casper_contract_schema_version` - Version of the schema.
- `toolchain` - Toolchain used to compile the contract.
- `contract_name` - Name of the contract.
- `contract_version` - Version of the contract.
- `types` - List of custom types used in `entry_points` and `events`.
- `entry_points` - List of entry points.
- `events` - List of events.

### Types
Types section is used to defined all the custom defined types in `entry_points`
and `events`. Custom types can be constructed out of CLTypes and other custom types.

There are two kinds of types:
- `struct` - Represents a struct kind of data with a list of named fields.
- `enum` - Represents an enum type with a list of named variants.

> Casper 2.0 no longer plans to use the CLType format. Yet it should be still
> used here because CLType is implementend already in all SDKs. For 2.0 we might
> prepare next version that has different type format.

### Entry Points
Entry points are the functions that can be called on the contract. Each entry
has following properties:
- `name` - Name of the entry point.
- `is_mutable` - Whether the entry point is mutable or not.
- `is_payable` - Whether the entry point is payable or not.
- `args` - List of named arguments to the entry point.
- `return_ty` - Return type of the entry point.
- `contract_context`: If `true` then it is contract context, otherwise session
  context.

### Events
Events are a list of named types. Each event has following properties:
- `name` - Name of the event.
- `ty` - Type of the event.

It is design to allow reuse the same type with different names.
