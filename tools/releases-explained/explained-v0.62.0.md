# Release: SpaceCraft SDK v0.62.0

Date: 2025-09-24

## Short description:

SpaceCraft v0.62.0 provides BLS cryptographic signing in tests, as well as gas simulation functionality.


## Full description:

### BLS signing support

BLS support in the Rust VM and tests is already available since v0.61.0, but only the validation. This release adds the ability to sign the messages in the tests. This makes it easier to test complex scenarios.

### Gas simulation

It is now possible to simulate transactions before sending them from interactors, in order to estimate the gas consumption.



The SpaceVM offer many built-in cryptographic functions to smart contracts. One of them is the BLS category. Until now they couldn't be used in blackbox and whitebox tests, because they were not implemented in the Rust VM.

Implementing them turned out trickier than expected, since we couldn't find an exact Rust equivalent of the library used in the Go VM. Therefore, we needed to heavily adapt an existing one ([herumi/bls-eth-rust](https://github.com/herumi/bls-eth-rust)) as [mx-bls-rs](https://github.com/multiversx/mx-bls-rs).

We also performed some fixes with the interactor logs.
