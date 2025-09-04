# Release: SpaceCraft SDK v0.57.0

Date: 2025-04-04

## Short description:

Release v0.57.0 adds Windows support, cleans up some framework types, adds a reset mechanism for back transfers, and improves chain simulator testing.


## Full description:


### Windows support

Historically, we used to provide no support for Windows, due to an incompatibility of the VM executor. However, even though running a node on windows is not supported, other operations such as building contracts and deploying them to a blockchain have no such restriction.

This is the first release that supports developing and launchin transactions on Windows.


### Improved framework types

- ManagedVec payloads work better, and for any size. Enums with fields can be used in lists.
- ManagedDecimal works better, and more operations are supported.
- Bitflag types are supported in contracts.

These improvements make working with certain types easier for developers.

### Back transfer reset

A reset mechanism for back-transfers was added, which is useful when multiple sync calls occur in the same transaction.

### Set state overwrite support in the chain simulator

It is now possible to overwrite the initial state in the chain simulator, when running chain simulator tests.


### Various fixes & optimizations:

- Rust 1.85 compiler support & optimizations.
- Token transfer role fix.
- Improved error messages.
