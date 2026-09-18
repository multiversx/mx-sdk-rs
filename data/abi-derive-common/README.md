# multiversx-sc-abi-derive-common

Common logic for the `TypeAbi` derive macros in MultiversX smart contracts.

This is a regular library crate (not a proc-macro crate) that contains the shared implementation used by both `multiversx-sc-abi-derive` and `multiversx-sc-derive`.

## Contents

- **`type_abi_derive`** — generates `TypeAbi` trait implementations for structs and enums
- **`parse`** — attribute parsing utilities (doc comments, macro attributes)
