# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-10-20

### Added
- `storage` cargo feature that adds a `storage` section to the schema root,
  describing where each field of a module lives in the contract storage.
- `StorageKind`, `StorageField` and `KeyEncoding` types (behind the `storage`
  feature) modelling the storage layout as a tree of `value`, `mapping`, `list`,
  `sequence`, `module`, `named_key` and `dictionary` nodes.
- `dns_schema_with_storage.json` and `erc20_schema_with_storage.json` examples.
- README section documenting the storage layout.

## [0.8.0] - 2026-10-12

### Added
- `toolkit` field on `ContractSchema` with a new `Toolkit { name, version }`
  struct describing the toolkit the contract was built with.
- `is_payable` flag on `Entrypoint`.
- Updated `dns` and `erc20` example schemas with the new fields.

## [0.7.0] - 2026-09-14

### Changed
- Bumped `casper-types` to `7`.

## [0.6.0] - 2025-09-10

### Added
- `Clone` and `Hash` derives for `StructMember`, `Type`, `TypeName` and
  `CustomType`.
- `CustomType::name()` helper returning the name of a struct or enum type.
- Custom `Deserialize` implementation for `NamedCLType`, so schemas can be read
  back from JSON.

## [0.5.0] - 2025-07-03

### Changed
- Bumped `casper-types` to `6`.

## [0.4.1] - 2025-06-06

### Changed
- Reverted `casper-types` to `5`.
- Bumped `schemars` to `0.9`.

## [0.4.0] - 2025-06-03

### Changed
- Bumped `casper-types` to `6`.

## [0.3.0] - 2025-04-24

### Changed
- Migrated to Casper 2.0: `casper-types` is now the published `5` release.
  During development the crate tracked the `feat-2.0` branch and the
  `2.0.0-rc5` and `2.0.0-rc9` pre-releases of `casper-node`.
- Pinned `schemars` to `0.8.18`.

## [0.2.0] - 2024-05-14

### Changed
- Bumped `casper-types` to `4.0.1`.

## [0.1.0] - 2024-03-27

### Added
- Initial `casper-contract-schema` crate with `ContractSchema`, `Entrypoint`,
  `Argument`, `Event`, `CallMethod`, `Type`, `CustomType` and `NamedCLType`
  data structures.
- `casper_contract_schema_version` field in the schema root.
- Support for enum custom types.
- `authors`, `repository`, `homepage` and `errors` fields in the schema root.
- Documentation comments on all public types.
- `Debug` derives on all structs and enums.
- `dns` and `erc20` example contracts with generated schemas.

### Changed
- Renamed the `contract_context` flag to `is_contract_context`.
- Widened enum variant and user error discriminants from `u8` to `u16`.
- Moved the crate sources from `casper-contract-schema/` to the repository root.
