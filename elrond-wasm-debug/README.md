# elrond-wasm-debug

The crate is necessary for smart contract debug projects.

It provides mocks for the entire blockchain infrastructure, so no call to the VM is necessary. In debug mode the VM is merely simulated.

For convenience, elrond-wasm-debug and subsequently all debug crates that use it are not #[no-std].
