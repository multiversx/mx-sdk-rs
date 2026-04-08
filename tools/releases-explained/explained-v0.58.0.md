# Release: SpaceCraft SDK v0.58.0

Date: 2025-05-26


## Short description:

This release unlocks native execution of smart contracts in blackbox tests for the first time. That means you can now test your code against real contracts, even without their source, and simulate realistic, multi-contract behavior. 

It also adds Rust 1.87 support, experimental gas metering, Wasmer 6 support, and a streamlined build system.


## Full description:

SpaceCraft SDK v0.58.0 is live, and testing just got a lot more real.

This release was 3+ months in the making, with over 27,000 lines of code rewritten to open up testing workflows for real-world complexity.

### Rust VM Wasmer integration

For the first time ever, developers can run actual smart contracts in blackbox tests, no source code needed. That means better testing for contracts you don’t own, don’t control, or just don’t want to peek into.

Why this matters:
- Run tests against any Wasm contract, no matter how it was built
- Simulate multi-contract interactions more realistically
- Prepare for production-level usage with real code
- Metering is now possible in Rust integration tests

This release also does some preparation work for the future integration of Wasmer 6 (or subsequent version) into the main Go VM. Even though Wasmer 6 is currently only available for developers in the Rust VM, much of the groundwork has been laid for a deeper integration.


### Support for Rust 1.87

Rust 1.87 has migrated to LLVM 20, which by default adds memory copy and fill opcodes to smart contracts. These opcodes are not yet supported by the MultiversX Space VM.

To avoid issues, the build system uses a custom build mode to avoid these opcodes (`wasmv1-none` instead of `wasm32-unknown-unknown`).

To make builds easier, both these targets will be auto-installed on build, as required.

All these build configurations are made custimzable in `sc-config.toml`.

Note: Rust 1.87 (or newer) cannot be used with smart contracts built with older versions of the framework (< 0.58.0).


### Opcode validator

An opcode validator is now run by default after every build. It detects forbidden opcocdes, including the problematic memory.copy and memory.fill mentioned above.
