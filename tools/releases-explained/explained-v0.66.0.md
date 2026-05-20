# Release: SpaceCraft SDK v0.66.0

Date: 2026-05-21


## Short description:

SpaceCraft v0.66.0 is a major release with several landmark additions. The highlights are a new sc-meta transaction CLI for composing and broadcasting transactions from the terminal, a comprehensive reproducible-build toolchain, correct Drop implementations for managed types that fix long-standing memory leaks, and a rich set of mathematical library additions including ManagedDecimal improvements and new BigUint utilities.


## Full description:

### Overview

v0.66.0 is the largest release in the v0.66 cycle. It touches almost every layer of the stack: the smart contract framework gains correct memory management for managed types and new mathematical primitives, the sc-meta tooling gains two entirely new command groups (tx and reproducible-build), the chain VM receives gas schedule updates, and the SDK is refactored to align naming with the Go SDK.

This document walks through each area in turn.


### Memory and thread safety for managed types

Perhaps the most consequential correctness fix in this release is the introduction of proper `Drop` implementations for managed types.

Previously, managed types such as `ManagedBuffer`, `BigInt`, `BigFloat`, `ManagedVec`, and `ManagedMap` did not call their VM-side destructor when dropped in Rust. This is only relevant in `StaticApi` contexts — that is, in tests, interactors, and Rust services — where the VM heap is managed by the Rust process itself. In those contexts, failing to call the destructor caused memory leaks and, in some edge cases, double-drop bugs. Both issues are now resolved: each of these types calls the appropriate VM hook when its Rust value is dropped.

Alongside the destructor fix, the `ManagedVecItem` derive macro was updated to generate the `requires_drop` flag correctly, a slice out-of-bounds bug in `ManagedVec` was fixed, and a bug in `MultiValueEncoded::to_arg_buffer` was corrected. Memory benchmarks were added to track managed type allocations over time.

On the thread-safety side, `DebugHandle` and `StaticApiHandle` are now explicitly `!Send`, and all managed types implement `!Send + !Sync`. Tests enforce this at compile time. Managed types were never safe to share across threads, but the marker traits now make the compiler enforce it.


### Mathematical library additions

v0.66.0 adds a substantial set of mathematical utilities to the base framework.

`BigUint` gains:
- `nth_root`: integer nth-root computation.
- `SaturatingSub` and `SaturatingSubAssign`: subtraction that saturates at zero rather than panicking on underflow.
- `FromStr` / `parse()`: construct a `BigUint` from a decimal string, implemented via the standard `FromStr` trait.

Two new standalone functions are introduced:
- `linear_interpolation`: computes a linearly interpolated value between two endpoints.
- `weighted_average`: computes a weighted average of a set of values.

`ManagedDecimal` receives the most additions of any single type:
- `exp_approx`: an exponential approximation function.
- `compounded_interest` and `compounded_interest_factor`: methods for compound-interest calculations.
- `mul` and `div` with half-up rounding mode, complementing the existing truncating variants.
- `nth_root`: nth-root support, matching `BigUint`.
- Fixes to `into_raw_units` and `as_raw_units` conversions that previously produced incorrect results in some cases.
- Backwards-compatibility fixes to ensure existing code continues to work after the other changes.


### sc-meta: transaction CLI (sc-meta tx)

A new top-level command group, `sc-meta tx`, has been added. It provides a command-line interface for composing, signing, and broadcasting transactions directly from the terminal, without writing a separate interactor or script. The command set is modelled after `mxpy`, the Python-based MultiversX CLI, with the goal of eventually offering a pure-Rust alternative. Not all mxpy features have been migrated yet; this release covers the core transaction workflows.

The available subcommands are:

- `sc-meta tx deploy` — deploy a contract.
- `sc-meta tx call` — call an endpoint on a deployed contract.
- `sc-meta tx query` — query a contract endpoint without consuming gas.
- `sc-meta tx sign` — sign a transaction offline and write the signed payload to a file.
- `sc-meta tx upgrade` — upgrade a deployed contract to a new code version.

Common flags across the subcommands include:
- `--payments` / `--token-transfers` for attaching ESDT token transfers to a transaction.
- `--wait-result` (requires `--send`) to block until the transaction result is available.
- `--code-metadata` arguments were refactored for clarity.

After a transaction is submitted, the explorer URL for the transaction is printed to the console. The same URL is also displayed in interactor workflows and whenever a new contract is deployed.


### sc-meta: reproducible builds (sc-meta reproducible-build)

The `sc-meta reproducible-build` command group provides tooling for building contracts in a reproducible environment and verifying that a deployed contract matches a given source tree.

Key capabilities:

- **Docker-based local build**: contracts are compiled inside a Docker container with a fixed toolchain, ensuring the output Wasm is identical regardless of the host machine.
- `sc-meta all download` with an `--overwrite` flag for refreshing previously downloaded artifacts.
- `sc-meta all publish` and `unpublish` with polling and a configurable maximum-attempt limit, for interacting with a contract registry.
- Source pack/unpack (zip): the source tree can be packed into a zip for distribution and unpacked for verification, with full test coverage.
- `artifacts.json` generation and code-hash verification: after a build, a manifest of output files and their hashes is produced. A subsequent verification step compares the deployed code hash against this manifest.
- `repro-build init-config`: generates a project-level configuration TOML that records the Docker image and other settings used for the reproducible build.
- `repro-build release-notes` CLI: generates release notes from the build artifacts.
- Integration test and CI support: the tooling includes integration tests and hooks for running reproducible-build verification in CI pipelines.


### sc-meta: other improvements

Several other sc-meta commands received updates:

- **Wallet**: `sc-meta wallet new` now hides the password input from the console using `rpassword`, preventing accidental password exposure. A new `sc-meta wallet test-wallet` command was added for verifying wallet files.
- **Data CLI**: `sc-meta data` is a new subcommand for reading and manipulating contract storage, intended for scripting use cases.
- `sc-meta new --force` flag: overwrites an existing template directory without prompting. A related `--overwrite` bugfix was also included.
- `sc-meta install wasm32` gains a `--toolchain` flag for specifying which Rust toolchain to target.
- `sc-meta install debugger` on Windows now has duplicate-protection and configures `rust-analyzer.debug.engine` automatically.
- `sc-meta install all` was fixed on Windows.
- `sc-meta all codehash` now has a fallback for contracts where the hash cannot be computed directly.
- Duplicate contract names are now detected and reported as an error.
- `sc-meta rust version` is now displayed in full.
- The `CARGO_NET_GIT_FETCH_WITH_CLI` environment variable is now propagated through contract builds, which is required in some corporate network environments.


### Rust VM gas schedule updates

The Rust VM received gas schedule updates for gas schedule v9 and above. Specific changes include:

- Fixed gas accounting for type conversions and additional VM hooks.
- Gas accounting for `ManagedMap` operations was added.
- An overflow check was added to the multiply-gas helper, preventing silent miscalculations.
- `wasmer-prod` received a fix for a missing breakpoint after an early exit or out-of-gas condition, which could previously cause incorrect execution flow.


### Chain core additions

The `multiversx-chain-core` crate received several additions:

- A standard code hash function is now part of chain core, with a corresponding VM hook for reading a contract's code hash on-chain.
- Basic crypto functions were consolidated into chain core, centralizing what was previously scattered across crates.
- Deploy address computation logic was centralized in chain core.

`Bech32Address` received targeted improvements:
- A `try_from_bech32_string` constructor that returns an explicit error rather than panicking on invalid input.
- A `FromStr` implementation, making `Bech32Address` parseable via the standard `str::parse()` mechanism.
- A clearer error message when an empty string is passed as input.


### SDK improvements

The SDK was refactored and extended:

- Interactor gas price support: interactors can now specify a gas price for transactions, rather than relying solely on the default.
- REST API types and naming were updated to match the Go SDK implementation, improving consistency for developers working across both SDKs.
- `Wallet::from_pem_file` and related `Wallet` methods now accept any `AsRef<Path>` argument instead of a concrete `&Path`, making them easier to use with `String`, `PathBuf`, and other path types.


### ManagedByteArray display

`ManagedByteArray` now implements `SCDisplay` and `SCBinary`, allowing fixed-size byte arrays to be printed in display and binary format. This is useful for logging raw byte sequences in contract output and in tests.


### Dependency upgrades

Various dependencies were updated to their latest versions.
