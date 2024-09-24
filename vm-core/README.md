# MultiversX VM base types, interfaces and builtin function names

It provides various types and contants referring to the MultiversX blockchain base implementation.

This functionality is designed to be minimal and to be used from both smart contract code and VM implementations.

It can be viewed as a collection of system specs, which hold for any MultiversX-related implementation. For example:
- `Address` - MultiversX adresses are 32 bytes long. This is the old SC address type, it holds the bytes on the heap. It is also used in the MultiversX Rust VM.
- `H256` - same as address, currently used for transaction hashes.
- Flags:
    - Code metadata - a bitflag encoding the SC code metadta, as it is stored on the blockchain, and encoded in smart contracts;
    - ESDT local roles
        - as enum
        - as bitflags
    - ESDT token types

