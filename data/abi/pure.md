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

### 3. `HasUnmanaged` — framework result compatibility

```rust
/// The historical concrete result type used by `ReturnsResultUnmanaged`.
pub trait HasUnmanaged: TypeAbi {
    type Unmanaged;
}
```

This trait lives in `framework/base`, next to its only consumer. It is implemented on concrete
`TypeAbi` types so that old mappings remain intact. Despite the name, `Unmanaged` can be a managed
type when that was the historical behavior. `ReturnsResultUnmanaged` bounds `Original: HasUnmanaged`.

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
```

`ListAbi<T>` — generic:
```rust
impl<T: AbiType> AbiType for ListAbi<T> {
    fn type_name() -> TypeName { /* "List<T>" or "bytes" */ }
    // ...
}
impl<T: AbiType> TypeAbi for ListAbi<T> {
    type Abi = Self;
}
```

`Vec<T>`, `&[T]`, `Box<[T]>`, and `ArrayVec<T, CAP>` all use `ListAbi<T::Abi>`.
`String`, `&'static str`, and `Box<str>` all use `StringAbi`. Ordinary variadic
containers use `VariadicAbi<T::Abi>`.

### B. Primitives and standard types (`type_abi_impl_basic.rs`)

Primitive ABI owners, `()`, arrays, tuples, and `Option<T>` implement `AbiType`.
Concrete representation types use their shared descriptors instead: strings use `StringAbi`,
lists use `ListAbi<T>`, `usize` uses `u32`, and `isize` uses `i32`.

```rust
impl AbiType for u32 { fn type_name() -> TypeName { "u32".into() } }
impl TypeAbi for u32 { type Abi = Self; fn type_name_rust() -> TypeName { "u32".into() } }
```

`&T` — transparent wrapper, does NOT implement `AbiType`:
```rust
impl<T: TypeAbi> TypeAbi for &T {
    type Abi = T::Abi; // was Self — corrected to delegate to T's ABI type
    fn type_name_rust() -> TypeName { T::type_name_rust() }
}
```

### C. VM-core types (`type_abi_impl_vm_core.rs`)

Self-describing VM-core types such as `H256` and `CodeMetadata` implement `AbiType` themselves.
`BoxedBytes` uses `BytesAbi`; `BLSKey` and `BLSSignature` use their fixed byte-array ABI types.
`Address` keeps `type Abi = AddressAbi` (separate ABI marker already exists).

### D. Managed framework types (`framework/base/src/**`)

`BigUint<M>`, `BigInt<M>`, etc. do NOT implement `AbiType`. They only implement `TypeAbi`:

```rust
impl<M: ManagedTypeApi> TypeAbi for BigUint<M> {
    type Abi = BigUintAbi; // BigUintAbi implements AbiType
    fn type_name_rust() -> TypeName { "BigUintAbi".into() }
}
impl<M: ManagedTypeApi> HasUnmanaged for BigUint<M> {
    // Feature-gated exactly as the historical TypeAbi::Unmanaged mapping was.
    type Unmanaged = /* Rust BigUint or Self */;
}
```

---

## File-by-File Changes

| File | Changes |
|---|---|
| `types/type_abi.rs` | Add `AbiType` trait; modify `TypeAbi` (remove `Unmanaged`, `type_name`, update `type_names`) |
| `types/type_abi_from.rs` | Add `AbiTypeFrom<Source: AbiType>`; keep `TypeAbiFrom` for compat |
| `framework/base/src/types/has_unmanaged.rs` | Define `HasUnmanaged` and compatibility mappings |
| `types/pure_abi/*.rs` | Define canonical ABI descriptors, including lists, strings, and variadics |
| `types/type_abi_impl_basic.rs` | Map concrete standard representations to shared ABI descriptors |
| `types/type_abi_impl_vm_core.rs` | Add `AbiType` for self-describing types; update `TypeAbi` |
| `types/type_abi_impl_codec_multi.rs` | Update `TypeAbi` |
| `types/type_abi_impl_big_int.rs` | Update `TypeAbi` |
| `lib.rs` | Export `AbiType`, `AbiTypeFrom` |
| `framework/base/src/**` | Move historical `Unmanaged` mappings to framework-owned implementations |
| `result_handlers/returns_result_unmanaged.rs` | Bound: `Original: HasUnmanaged` |
| `result_handlers/returns_result_as.rs` | Bound: `T::Abi: AbiTypeFrom<Original>` instead of `T: TypeAbiFrom<Original>` |

---

## Open Questions

- **Cross-type `TypeAbiFrom` migration**: managed→ABI edges like `impl TypeAbiFrom<u64> for BigUint<M>` become `impl AbiTypeFrom<u64> for BigUintAbi`. Audit all cross-type edges during migration.
- **`ProxyArg<O>` update**: once `AbiTypeFrom` is stable, `O: TypeAbiFrom<T>` becomes `O: AbiTypeFrom<T::Abi>`.
