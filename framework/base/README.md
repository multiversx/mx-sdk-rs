# Smart contract base crate

[![crates.io](https://img.shields.io/crates/v/multiversx-sc.svg)](https://crates.io/crates/multiversx-sc) 

This is the main crate for building smart contracts for the MultiversX blockchain.

It contains the interface that the smart contract sees and can use. No implementation details are available from this crate alone.

# no-std

The crate supports both std and no-std builds. Building for the blockchain is done with no-std, while for debugging std is used.

Using a heap allocator is possible but discouraged. Use the "alloc" feature if you need it.
