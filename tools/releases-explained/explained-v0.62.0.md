# Release: SpaceCraft SDK v0.62.0

Date: 2025-09-24

## Short description:

SpaceCraft v0.62.0 provides BLS cryptographic signing in tests, as well as gas simulation functionality.


## Full description:

### BLS signing support

BLS support in the Rust VM and tests is already available since v0.61.0, but only the validation. This release adds the ability to sign the messages in the tests. This makes it easier to test complex scenarios.


### Gas simulation

It is now possible to simulate transactions before sending them from interactors, in order to estimate the gas consumption.

It is also possible to perform just the simulation, using the same syntax.


### Post-build check improvements

The framework does some post-build checks to ensure smart contract correctness. An incorrect smart contract will be rejected by the blockchain, but it is nice to be aware of issues before trying to deploy.

Note that the framework doesn't normally produce incorrect contracts, it is rare that this post-build check catches anything.

Not all WebAssembly opcodes are supported by the MultiversX VM. For instance, newer versions of Rust emit memory.copy and memory.fill opcodes, which are not yet supported. The checker can catch them, but it had a bug that was preventing it from tracing them via indirect calls. The issue was discovered while compiling contracts using the standard library. This is nowadays possible, but very uncommon.

In preparation for the post-Supernova release, we are versioning the opcode whitelist, and preparing support for more opcodes. Even though this release is in the future, we have already added support in the framework.

We have also added a check for VM hook signatures. It did not detect any issues and it is unlikely it ever does, but it represents an additional layer of security nonetheless.

### Proxy generator fix for enums with explicit discriminants

Rust enums can have explicit discriminants. These were causing the proxy generator to fail, before this release.

### Removed legacy transaction syntax

Before the unified transaction syntax, there used to be a Mandos-based syntax for tests and interactors. It was unsuccessful, and barely used. At that point the oldest testing framework was still popular and remained widespread.

People only started to migrate tests once the unified syntax was introduced. The mandos-based solution has been deprecated for more than 2 years, and we are not aware of anyone using it. To simplify our codebase, we removed some of this code.

The old transaction structures are also pending deletion. For now, they are hidden behind a feature flag.
