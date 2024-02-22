# Casper Contract Schema

Welcome to our Casper Contract Schema repository! This is where we're building a
standard way to connect smart contracts with the world on the Casper network.
It's all about making blockchain apps work better together, and we need your
help to make it even better. Using the Casper Contract Schema means your
blockchain projects will be more compatible and easier to develop.

## How you can help

We're looking for feedback on the schema, and we'd love to hear from you. If you
have any ideas, suggestions, or questions, please open an issue or a pull
request.

## Examples
See [erc20_schema.json] and [dns_schema.json] for an example usages.

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
- `enum` - Represents an enum type with a list of named variants. There three modes when defining variants:
    - `single unnnamed field` - variant with a single unnamed field is encoded as is,
    - `multiple unnamed fields` - variant with multiple unnamed fields are packed into a tuple,
    - `named fields` - variant with named fields are encoded as a struct.

See [dns_schema.json] for all the examples.

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

[dns_schema.json]: ./casper-contract-schema/examples/dns_schema.json
[erc20_schema.json]: ./casper-contract-schema/examples/erc20_schema.json
