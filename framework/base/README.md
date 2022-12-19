# elrond-wasm

This is the main crate for building smart contracts on Elrond's Arwen VM in Rust.

It contains the interface that the smart contract sees and can use. No implementation details are available from this crate alone.

# no-std

The crate supports both std and no-std builds. Building for the blockchain is done with no-std, while for debugging std is used.
