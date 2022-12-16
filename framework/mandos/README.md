# mandos

Rust implementation of the Mandos smart contract test file format.

It is composed of 2 parts:
- the mandos serde representation
- the standard mandos value interpreter

Both of them are detailed under this specification: https://docs.elrond.com/developers/developer-reference/mandos-tests

This crate only deals with the format, not with its semantics or execution. For the execution engine, see `elrond-wasm-debug/mandos`. This also means that this crate does not and should not depend on any `elrond-*` crate, it is the base format and nothing else.
