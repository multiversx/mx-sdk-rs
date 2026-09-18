# multiversx-sc-abi

ABI (Application Binary Interface) generation and type metadata for MultiversX smart contracts.

## Overview

This crate provides the core types and traits for generating ABI metadata in MultiversX smart contracts. The ABI describes:
- Contract endpoints (initialization, upgrades, regular endpoints, callbacks)
- Endpoint parameters and return types
- Events and their data
- ESDT attribute structures
- Custom type descriptions (structs and enums)

## Features

### Core Traits

- **`TypeAbi`** - Main trait implemented by all types that can appear in the ABI (arguments, results, event logs, etc.)
- **`TypeAbiFrom`** - Trait for type conversions in ABI context

### ABI Components

- **`ContractAbi`** - Complete contract ABI including endpoints, events, and type descriptions
- **`EndpointAbi`** - Metadata for contract endpoints (init, upgrade, view, external)
- **`EventAbi`** - Event definitions with indexed and non-indexed fields
- **`EsdtAttributeAbi`** - ESDT token attribute descriptions
- **`BuildInfoAbi`** - Build information (framework version, rustc version, etc.)

### Type Descriptions

- **`TypeDescription`** - Detailed description of custom types
- **`TypeContents`** - Enum/Struct field information
- **`TypeNames`** - ABI and Rust type name mappings
- **`TypeDescriptionContainer`** - Accumulator for collecting type descriptions

## Usage

### Implementing TypeAbi

For custom types, use the `#[type_abi]` attribute from the [`multiversx-sc-abi-derive`](../abi-derive) crate:

```rust
use multiversx_sc_abi::TypeAbi;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct MyStruct {
    pub field1: u64,
    pub field2: BigUint,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum MyEnum {
    Variant1,
    Variant2(u32),
    Variant3 { x: u64, y: u64 },
}
```

### Accessing Type Information

```rust
use multiversx_sc_abi::{TypeAbi, TypeDescriptionContainerImpl};

// Get type name
let name = MyStruct::type_name();

// Collect type descriptions for ABI generation
let mut accumulator = TypeDescriptionContainerImpl::new();
MyStruct::provide_type_descriptions(&mut accumulator);
```

### Built-in TypeAbi Implementations

The crate provides `TypeAbi` implementations for:

- **Primitive types**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `isize`, `bool`, `f64`
- **Core types**: `NonZeroUsize`
- **Codec types**: `TopEncodeMulti`, `TopDecodeMulti`, `MultiValueEncoded`, etc.
- **VM core types**: Address, token identifiers, managed types (when available)
- **BigInt types**: `num_bigint::BigUint`, `num_bigint::BigInt` (enabled with `num-bigint` feature)

## Features

- **`num-bigint`** - Enables TypeAbi implementations for `num_bigint::BigUint` and `num_bigint::BigInt`

## Dependencies

- [`multiversx-chain-core`](../../chain/core) - Core blockchain types
- [`multiversx-sc-codec`](../codec) - Binary serialization/deserialization
- `bitflags` - For bitflag type support
- `unwrap-infallible` - For infallible conversions

## Integration

This crate is typically not used directly in smart contracts. Instead:

1. Smart contract code uses types from `multiversx-sc` framework
2. The `#[type_abi]` attribute from `multiversx-sc-abi-derive` generates implementations
3. The `multiversx-sc-meta` tool uses this crate to generate ABI JSON files

## ABI Generation Flow

1. Contract code is annotated with `#[contract]`, `#[module]`, and `#[type_abi]` attributes
2. Procedural macros collect endpoint, event, and type information
3. `ContractAbi` structure is populated with all metadata
4. ABI is serialized to JSON for deployment and interaction tools

## Example ABI Output

```json
{
  "name": "MyContract",
  "endpoints": [
    {
      "name": "myEndpoint",
      "inputs": [
        {
          "name": "amount",
          "type": "BigUint"
        }
      ],
      "outputs": [
        {
          "type": "MyStruct"
        }
      ]
    }
  ],
  "types": {
    "MyStruct": {
      "type": "struct",
      "fields": [
        {
          "name": "field1",
          "type": "u64"
        },
        {
          "name": "field2",
          "type": "BigUint"
        }
      ]
    }
  }
}
```

## Related Crates

- [`multiversx-sc-abi-derive`](../abi-derive) - Procedural macros for deriving TypeAbi
- [`multiversx-sc-codec`](../codec) - Binary serialization used in ABI types
- [`multiversx-sc-meta`](../../framework/meta) - Tools that consume ABI for code generation

## License

GPL-3.0-only
