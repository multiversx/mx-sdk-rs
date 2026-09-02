# Release: SpaceCraft SDK v0.66.1

Date: 2026-06-01


## Short description:

SpaceCraft v0.66.1 is a patch release addressing a build compatibility issue with Rust 1.96, extending ManagedVecItem support to time types, and delivering a pair of sc-meta tooling fixes.


## Full description:

### Overview

v0.66.1 is a focused patch release. It restores WASM build compatibility with the Rust 1.96 toolchain, adds ManagedVecItem implementations for the time and duration types, and fixes two sc-meta issues: one in the template generated for interactors and snippets, and one in the rustc versioning logic.


### Build fix for Rust 1.96

Starting with Rust 1.96, extern C blocks targeting the wasm32 target no longer implicitly resolve to WebAssembly host imports. Each block must now carry a link attribute specifying the import module.

The wasm-adapter crate has been updated accordingly: all extern C declarations of VM API functions now carry the wasm_import_module = "env" attribute, restoring correct linking behaviour when building contracts with Rust 1.96 and later.


### ManagedVecItem for time types

The ManagedVecItem trait is now implemented for the four time and duration types: TimestampMillis, TimestampSeconds, DurationMillis, and DurationSeconds.

This makes it possible to store these types inside a ManagedVec and use them as generic payload types wherever ManagedVecItem is required, consistent with how primitive numeric types already behave.


### sc-meta fixes

Two issues in sc-meta were resolved.

The template generated for the interactor and the associated snippets.sh file contained an error that caused the downloaded contract template to be malformed. This has been corrected so that newly generated interactors and their shell scripts are immediately usable.

An edge case in the rustc versioning system has been fixed. When the active toolchain reported a patch version such as 1.85.1, the version string passed to rustup included the full patch component, causing rustup to fail with a missing manifest error. The version string used for rustup arguments now consistently uses only the major.minor form, which matches how rustup registers installed toolchains.
