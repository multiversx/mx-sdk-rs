# Release: SpaceCraft SDK v0.62.1

Date: 2025-10-27

## Short description:

SpaceCraft v0.62.1 fixes an issue with Rust VM BLS functionality.


## Full description:

The BLS functionality in the Rust VM is a wrapper around a C library. It seems to have issues with concurrent access, which compromises tests. We therefore added a Mutex to protect it.

It also fixes a dependency to `home`, via `wasmer`, which as of 0.5.2 requires Rust 1.88, which doesn't work with Wasmer on Linux machines (at least not yet).

Also fixed an issue with the deprecated legacy contract call module.
