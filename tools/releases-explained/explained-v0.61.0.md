# Release: SpaceCraft SDK v0.61.0

Date: 2025-09-04

## Short description:

SpaceCraft v0.61.0 introduces BLS cryptographic functions for testing smart contracts.


## Full description:

### Highlights:

The SpaceVM offer many built-in cryptographic functions to smart contracts. One of them is the BLS category. Until now they couldn't be used in blackbox and whitebox tests, because they were not implemented in the Rust VM.

Implementing them turned out trickier than expected, since we couldn't find an exact Rust equivalent of the library used in the Go VM. Therefore, we needed to heavily adapt an existing one ([herumi/bls-eth-rust](https://github.com/herumi/bls-eth-rust)) as [mx-bls-rs](https://github.com/multiversx/mx-bls-rs).

We also performed some fixes with the interactor logs.
