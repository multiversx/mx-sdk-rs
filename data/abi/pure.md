# AbiType Trait Split — Implementation Plan

## Motivation

`TypeAbi` currently serves two distinct roles:
- **Abstract type descriptor**: owns the ABI name (`type_name`), JSON schema generation
- **Concrete type link**: maps a Rust type to its ABI descriptor (`type Abi`, `type_name_rust`)

This plan splits them cleanly. `Unmanaged` is extracted into a third, optional trait because not all pure ABI types have a concrete default implementation available (e.g. `BigUintAbi` is feature-gated on `num-bigint`).

---

## New Trait Definitions

### 1. `AbiType` — abstract ABI descriptor

```rust
pub trait AbiType {
    fn type_name() -> TypeName;
    fn type_name_specific() -> Option<TypeName> { None }
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC);
    fn is_variadic() -> bool { false }
    fn output_abis(output_names: &[&'static str]) -> OutputAbis;
}
```

Implemented by: all `...Abi` marker types, primitives (`u32`, `bool`, …), `H256`, framework-agnostic types.  
NOT implemented by: managed types with a type parameter (`BigInt<M>`, `EgldOrEsdtTokenIdentifier<M>`).

### 2. `AbiTypeFrom<Source: AbiType>` — ABI-level compatibility

```rust
/// "Self can accept/decode Source in ABI terms."
pub trait AbiTypeFrom<Source: AbiType>: AbiType {}

impl<T: AbiType> AbiTypeFrom<T> for T {} // every ABI type accepts itself

// Cross-type edges migrate here from TypeAbiFrom:
impl AbiTypeFrom<u8>  for BigUintAbi {}
impl AbiTypeFrom<u16> for BigUintAbi {}
// etc.
```

The old `TypeAbiFrom<T>` is kept for backward compatibility; its cross-type impls migrate here.

### 3. `HasUnmanaged` — optional default concrete type

```rust
/// A pure ABI type that has a known default concrete Rust implementation.
/// Not implemented when the concrete type is feature-gated or unavailable.
pub trait HasUnmanaged: AbiType {
    type Unmanaged: TypeAbi;
}
```

`ReturnsResultUnmanaged` is updated to bound `Original: AbiType + HasUnmanaged`.

### 4. Modified `TypeAbi` — concrete type with ABI link

`type Unmanaged` and `type_name()` are removed. `type Abi` is now bounded by `AbiType`. `type_names()` delegates `type_name` to `Self::Abi`.

```rust
pub trait TypeAbi: TypeAbiFrom<Self> {
    type Abi: AbiType;

    fn type_name_rust() -> TypeName;

    fn type_names() -> TypeNames {
        TypeNames {
            abi: Self::Abi::type_name(),
            rust: Self::type_name_rust(),
            specific: Self::Abi::type_name_specific(),
        }
    }
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        Self::Abi::provide_type_descriptions(accumulator);
    }
}
```

---

## Migration by Category

### A. Pure ABI marker types (`data/abi/src/types/pure_abi/*.rs`) — 15 files

`AddressAbi`, `BigFloatAbi`, `BigIntAbi`, `BigUintAbi`, `DecimalAbi`,
`DecimalSignedAbi`, `EgldOrEsdtTokenIdentifierAbi`, `EllipticCurveAbi`,
`EsdtTokenIdentifierAbi`, `FungiblePaymentAbi`, `ListAbi<T>`, `NonZeroBigUintAbi`,
`PaymentAbi`, `SignAbi`, `TokenIdAbi`.

Non-generic pattern:
```rust
impl AbiType for BigIntAbi {
    fn type_name() -> TypeName { "BigInt".into() }
}
impl TypeAbi for BigIntAbi {
    type Abi = Self;
    fn type_name_rust() -> TypeName { "BigIntAbi".into() }
}
impl HasUnmanaged for BigIntAbi { type Unmanaged = Self; }
```

`BigUintAbi` — feature-gated `HasUnmanaged`:
```rust
#[cfg(not(feature = "num-bigint"))]
impl HasUnmanaged for BigUintAbi { type Unmanaged = Self; }

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for BigUintAbi {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}
```

`ListAbi<T>` — generic:
```rust
impl<T: AbiType> AbiType for ListAbi<T> {
    fn type_name() -> TypeName { /* "List<T>" or "bytes" */ }
    // ...
}
impl<T: TypeAbi> TypeAbi for ListAbi<T> {
    type Abi = ListAbi<T::Abi>; // propagates ABI type through the container
    fn type_name_rust() -> TypeName { /* */ }
}
```

### B. Primitives and standard types (`type_abi_impl_basic.rs`)

`u8`…`i128`, `bool`, `()`, `String`, `&'static str`, `Box<str>`, arrays, tuples, `Vec<T>`, `Option<T>`.

```rust
impl AbiType for u32 { fn type_name() -> TypeName { "u32".into() } }
impl TypeAbi for u32 { type Abi = Self; fn type_name_rust() -> TypeName { "u32".into() } }
impl HasUnmanaged for u32 { type Unmanaged = Self; }
```

`&T` — transparent wrapper, does NOT implement `AbiType`:
```rust
impl<T: TypeAbi> TypeAbi for &T {
    type Abi = T::Abi; // was Self — corrected to delegate to T's ABI type
    fn type_name_rust() -> TypeName { T::type_name_rust() }
}
```

### C. VM-core types (`type_abi_impl_vm_core.rs`)

`H256`, `BoxedBytes`, `CodeMetadata`, etc. are framework-agnostic and implement `AbiType` themselves.  
`Address` keeps `type Abi = AddressAbi` (separate ABI marker already exists).

### D. Managed framework types (`framework/base/src/**`)

`BigUint<M>`, `BigInt<M>`, etc. do NOT implement `AbiType`. They only implement `TypeAbi`:

```rust
impl<M: ManagedTypeApi> TypeAbi for BigUint<M> {
    type Abi = BigUintAbi; // BigUintAbi implements AbiType
    fn type_name_rust() -> TypeName { "BigUintAbi".into() }
}
// HasUnmanaged is on BigUintAbi, not BigUint<M>
```

---

## File-by-File Changes

| File | Changes |
|---|---|
| `types/type_abi.rs` | Add `AbiType` trait; modify `TypeAbi` (remove `Unmanaged`, `type_name`, update `type_names`) |
| `types/type_abi_from.rs` | Add `AbiTypeFrom<Source: AbiType>`; keep `TypeAbiFrom` for compat |
| NEW `types/has_unmanaged.rs` | Define `HasUnmanaged` trait |
| `types/pure_abi/*.rs` (15 files) | Add `AbiType` impl; update `TypeAbi`; add `HasUnmanaged` where appropriate |
| `types/type_abi_impl_basic.rs` | Add `AbiType` impls; update `TypeAbi`; fix `&T` to use `Abi = T::Abi`; add `HasUnmanaged` |
| `types/type_abi_impl_vm_core.rs` | Add `AbiType` for self-describing types; update `TypeAbi`; add `HasUnmanaged` |
| `types/type_abi_impl_codec_multi.rs` | Update `TypeAbi` |
| `types/type_abi_impl_big_int.rs` | Update `TypeAbi`; feature-gated `HasUnmanaged` |
| `lib.rs` | Export `AbiType`, `AbiTypeFrom`, `HasUnmanaged` |
| `framework/base/src/**` | All managed types: remove `type_name` overrides, remove `Unmanaged` |
| `result_handlers/returns_result_unmanaged.rs` | Bound: `Original: AbiType + HasUnmanaged` |
| `result_handlers/returns_result_as.rs` | Bound: `T::Abi: AbiTypeFrom<Original>` instead of `T: TypeAbiFrom<Original>` |

---

## Open Questions

- **`Box<T>` ABI type**: currently `type Abi = Box<T::Abi>`. Since `Box` is transparent at ABI level, consider simplifying to `type Abi = T::Abi`. Same question for all transparent container wrappers.
- **Cross-type `TypeAbiFrom` migration**: managed→ABI edges like `impl TypeAbiFrom<u64> for BigUint<M>` become `impl AbiTypeFrom<u64> for BigUintAbi`. Audit all cross-type edges during migration.
- **`ProxyArg<O>` update**: once `AbiTypeFrom` is stable, `O: TypeAbiFrom<T>` becomes `O: AbiTypeFrom<T::Abi>`.
