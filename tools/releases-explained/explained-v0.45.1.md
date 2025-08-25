# Release: SpaceCraft SDK v0.45.1

Date: 2023-11-24


## Short description:

- Replicated VM 1.5 in the Rust VM. This includes support for:
- promises,
- back-transfers,
- modified event logs.
- New endpoint annotation, `#[upgrade]`. Contract variants with upgrade endpoint, but without init now allowed.
- Build system:
    - `wasm` crates now fully generated based on data from `sc-config.toml` and root `Cargo.toml`.
    - Setting wasm target dir automatically, if not specified, based on workspace.
- Better hygiene in codec derive.


## Full description:

### Replicated VM 1.5 in the Rust VM

The new features of VM 1.5 are now also available in the Rust debugger: promises and back-transfers.

Some of the VM event logs (like “transferExecute”) were also changed in the VM. If needed, they behave the same in the debugger.

### New endpoint annotation, `#[upgrade]`

In VM 1.5, when upgrading smart contracts, the “init” function is no longer called, “upgrade” is called instead.

This new function can be annotated with `#[upgrade]`. It is currently just syntactic sugar for `#[endpoint(upgrade)]`, but more support for it will come.

### Build system improvements

The wasm crates of a contract are now all completely auto-generated. Until this release, the Cargo.toml file of the main wasm crate was being written by the developer. We included the build profile in the `sc-config.toml` profile, and so now nothing else build-related needs to be written by developer any longer.

To speed up the contract build process, the wasm target dir is now set automatically based on the enclosing workspace, if unspecified. Specifying it was tedious, and failing to specify it used to slow down the build process considerably.

We also cleaned up some of the dependencies in the `multiversx-sc-meta` crate, to make the builds faster.

### Better hygiene in codec derive

There was a problem when deriving any codec trait (TopEncode, TopDecode, NestedEncode, NestedDecode) over a structure or enum containing fields named “buffer”, “dest”, or “h”. Now fixed.