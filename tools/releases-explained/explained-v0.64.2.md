# Release: SpaceCraft SDK v0.64.2

Date: 2026-02-18


## Short description:

SpaceCraft v0.64.2 is a patch release that works around a VM hook bug affecting negative number serialization, adds new big number utilities, introduces the `FungiblePayment` type, upgrades Wasmer, and improves sc-meta error reporting and storage mapper documentation.


## Full description:


### Overview

This patch release contains a mix of bug fixes, new framework types, VM improvements, and developer experience enhancements. The most notable change is a workaround for a protocol-level VM hook bug that was silently corrupting negative numbers during serialization.


### 🐛 Workaround for `mBufferFromBigIntSigned` VM Hook Bug

A bug was discovered in the `mBufferFromSmallIntSigned` VM hook (also known as `mBufferFromBigIntSigned`), where negative numbers were being incorrectly converted to their absolute value. For example, writing `-5` to storage would silently store `5`.

#### The Root Cause

The bug originates in the Go VM implementation and affects all contract executions on-chain. The VM hook was discarding the sign of the number during the managed buffer conversion.

#### The Workaround

Since the framework cannot fix the protocol-level VM hook, the workaround avoids calling `mBufferFromSmallIntSigned` entirely. Instead, when serializing a signed small integer to a managed buffer, the framework now takes a two-step approach: first converting the value to a `BigInt`, and then converting the `BigInt` to a managed buffer via `mBufferFromBigIntSigned`, which handles signs correctly.

A feature flag `small-int-bug` was introduced to allow testing the buggy path, for verification purposes.

#### Deprecation

The `mBufferToBigIntSigned` VM hook has been added to the deprecated hooks list, so contracts will receive a warning if they rely on it directly.

#### Rust VM Fix

A related but separate bug was found in the Rust VM's `mb_to_small_int_signed` implementation. It was using unsigned byte parsing (`from_bytes_be` with `Sign::Plus`), which meant negative numbers read back from managed buffers were also incorrectly interpreted as positive. This was fixed by switching to `from_signed_bytes_be`. Additionally, the Rust VM's `mb_from_small_int_signed` now intentionally replicates the Go VM bug (converting to absolute value), so that Rust VM tests remain consistent with on-chain behavior until the protocol bug is resolved.


### 🧩 VM: ESDT Metadata Recreate and Metadata Update Mocks

Two new mock built-in functions were added to the Rust VM, contributed by the community (XOXNO):

- **ESDTMetaDataRecreate**: Replaces all metadata fields of an ESDT token instance unconditionally. All fields (name, royalties, hash, attributes, URIs) are overwritten.
- **ESDTMetaDataUpdate**: Applies a merge to the token metadata, only overwriting fields that have non-empty or non-zero new values.

Both functions validate that the caller is the token creator and emit appropriate transaction logs. This allows testing metadata management operations in the Rust VM without requiring a real chain.


### ⬆️ Upgraded `multiversx-chain-vm-executor` to v0.5.1

The executor crate was upgraded to v0.5.1, which includes an upgrade to Wasmer 6.1.0 (wasmer-experimental).

This primarily fixes a linker issue (`probestack`) that was causing wasmer-prod and wasmer-experimental builds to fail on Linux for certain Rust compiler versions. The `wasmer-prod` dependency was also updated to resolve a `home` crate version conflict.


### 🔢 Big Number Improvements

Several utilities were added to `BigInt` and `BigUint` to support common mathematical patterns in smart contracts:

#### `proportion` and `into_proportion`

These methods compute `self * part / total` efficiently. They are useful in financial calculations such as fee computation, reward distribution, or pro-rata splits.

For `BigUint`, `part` and `total` are `u64`. For `BigInt`, they are `i64`. The methods include overflow checks (panicking if `part` or `total` exceed `i64::MAX` in the `BigUint` case). Internally, they reuse a single temporary `BigInt` handle to minimize allocations.

`into_proportion` consumes the value, while `proportion` clones it first.

#### `BigUint::into_non_zero_or_panic`

A convenience method that converts a `BigUint` into a `NonZeroBigUint`, panicking if the value is zero. This is useful when a non-zero guarantee is needed at a specific point in the code.

#### `BigUint::new_unchecked`

An unsafe constructor that creates a `BigUint` from a `BigInt` without checking that the value is non-negative. This is intended for internal use in performance-sensitive paths where the non-negativity invariant is already guaranteed by the caller.

#### `BigInt::overwrite_i64` Signature Change

The `overwrite_i64` method now takes `&mut self` instead of `&self`. While this is a minor API change, it better reflects the mutation semantics and is used internally by the `proportion` implementation to reuse a temporary handle.


### 💰 New `FungiblePayment` Type

A new `FungiblePayment` type was introduced, representing a payment consisting of a `TokenId` and a `NonZeroBigUint` amount, without a token nonce. This is the natural type for fungible token transfers, where the nonce is always zero.

The type includes:
- `ManagedVecItem` implementation, so it can be stored in `ManagedVec` collections.
- `TypeAbi` implementation, for ABI generation and proxy support.
- Full codec support (top-level and nested encoding/decoding).
- Conversion methods: `into_payment()` to convert to a general `Payment`, and `Payment::fungible_or_panic()` to go the other direction (panicking if the nonce is non-zero).

This type was motivated by the order-book and digital-cash contract refactors, which needed a lighter-weight payment type for fungible-only operations.


### 🛠️ sc-meta Improvements

#### `--locked` CLI Flag

A new `--locked` flag was added to the build command. When specified, it passes `--locked` to the underlying `cargo build` invocation for the wasm crate, requiring that the `Cargo.lock` file is up to date. This is useful in CI environments to ensure reproducible builds.

#### Improved Build Error Messages

Build error messages now include the full command that was executed, making it much easier to reproduce and diagnose build failures. Previously, error messages would only mention that a command failed, without showing what exactly was run.


### 📚 Storage Mapper Documentation

Comprehensive documentation was added across 17 storage mapper implementations, including `SingleValueMapper`, `VecMapper`, `SetMapper`, `MapMapper`, `LinkedListMapper`, `QueueMapper`, `UserMapper`, `WhitelistMapper`, `UniqueIdMapper`, `BiDiMapper`, `OrderedBinaryTreeMapper`, `MapStorageMapper`, `AddressToIdMapper`, `UnorderedSetMapper`, `FungibleTokenMapper`, `NonFungibleTokenMapper`, and `TokenAttributesMapper`.

The documentation covers usage patterns, storage layout details, and behavioral notes for each mapper.


### 🔧 Derive Substitution List Fix

The derive preprocessing substitution list was corrected so that `Ref` and `ManagedVecRef` are no longer incorrectly listed as having an API generic type parameter. These types are concrete structs (not generic over `ManagedTypeApi`), so they do not need the automatic `Self::Api` injection that the derive macro applies to managed types. This prevents compilation errors when using these types in derived trait implementations.
