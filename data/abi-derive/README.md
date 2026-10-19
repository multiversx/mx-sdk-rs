# multiversx-sc-abi-derive

Procedural macros for deriving TypeAbi trait implementations in MultiversX smart contracts.

## Usage

This crate provides:
- `#[type_abi]` attribute macro for generating TypeAbi implementations
- `#[derive(TypeAbi)]` derive macro (deprecated, use #[type_abi] instead)

Place the `#[type_abi]` attribute before your derives:

```rust
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct MyStruct {
    pub field1: u64,
    pub field2: String,
}
```
