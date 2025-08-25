# Release: SpaceCraft SDK v0.58.0

Date: 2025-05-26


## Short description:

This release unlocks native execution of smart contracts in blackbox tests for the first time. That means you can now test your code against real contracts, even without their source, and simulate realistic, multi-contract behavior. 

It also adds Rust 1.87 support, experimental gas metering, Wasmer 6 support, and a streamlined build system.


## Full description:

SpaceCraft SDK v0.58.0 is live, and testing just got a lot more real.

This release was 3+ months in the making, with over 27,000 lines of code rewritten to open up testing workflows for real-world complexity.

For the first time ever, you can run actual smart contracts in blackbox tests, no source code needed. That means better testing for contracts you don’t own, don’t control, or just don’t want to peek into.

Why this matters:
- Run tests against any Wasm contract, no matter how it was built
- Simulate multi-contract interactions more realistically
- Prepare for production-level usage with real code

We’ve also:
- Enabled gas usage tracking (experimental for now)
- Added support for Rust 1.87
- Rolled in Wasmer 6 for local devs, while keeping compatibility with Wasmer 2.2 on mainnet
- Improved the build system with auto-target install + opcode validation

Coming soon: v0.59 with Barnard support on devnet. Stay tuned. 
