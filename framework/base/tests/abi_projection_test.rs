// Compile-time-only checks of `TypeAbi::Abi` projections: what the `.abi_typed`/
// `#[multiversx_sc_abi::contract_abi]` call-proxy generator actually turns each type into once it
// substitutes `Self::Api` with `UncallableApi` and projects through `<T as TypeAbi>::Abi` (see
// `data/abi-derive-common/src/contract/proxy_gen.rs`'s `abi_projected_type`).
//
// Each `#[test]` here asserts nothing at runtime; the only thing being checked is whether the
// file compiles. If a projection ever changes, this file stops compiling instead of some
// far-away blackbox test failing for a confusing reason.
//
// The types below mirror return types actually used in
// `contracts/feature-tests/payable-features/src/payable_features.rs`.

use multiversx_sc::abi::TypeAbi;
use multiversx_sc::api::uncallable::UncallableApi;
use multiversx_sc::imports::*;

type Api = UncallableApi;

/// `AssertTypeEq<T>` is only ever implemented for `T = Self` (the blanket impl below), so
/// `assert_type_eq::<A, B>()` only type-checks when `A` and `B` are the exact same type.
trait AssertTypeEq<T> {}
impl<T> AssertTypeEq<T> for T {}

fn assert_type_eq<A, B>()
where
    A: AssertTypeEq<B>,
{
}

/// `payable_all`'s return type. `Payment<M>` has a dedicated pure-ABI marker (`PaymentAbi`), so
/// the projection collapses the managed API parameter entirely.
#[test]
fn abi_projection_managed_vec_of_payment() {
    assert_type_eq::<<ManagedVec<Api, Payment<Api>> as TypeAbi>::Abi, ListAbi<PaymentAbi>>();
}

/// `payable_all_transfers`'s return type. Unlike `Payment<M>`, `EgldOrEsdtTokenPayment<M>` has no
/// dedicated pure-ABI marker: it's a plain `#[type_abi]` struct, whose derived `type Abi = Self`
/// leaves the managed API parameter in place. So the projection is a no-op pass-through, fixed at
/// whatever `UncallableApi` (the type-only source of `Env::Api` is not carried forward, only the
/// bare `Self::Api` reference is textually substituted).
#[test]
fn abi_projection_managed_vec_of_egld_or_esdt_token_payment() {
    assert_type_eq::<
        <ManagedVec<Api, EgldOrEsdtTokenPayment<Api>> as TypeAbi>::Abi,
        ListAbi<EgldOrEsdtTokenPayment<Api>>,
    >();
}

/// `payment_array_esdt_3`'s payment type. Same pass-through as `EgldOrEsdtTokenPayment` above:
/// `EsdtTokenPayment<M>` has no dedicated marker either.
#[test]
fn abi_projection_managed_vec_of_esdt_token_payment() {
    assert_type_eq::<
        <ManagedVec<Api, EsdtTokenPayment<Api>> as TypeAbi>::Abi,
        ListAbi<EsdtTokenPayment<Api>>,
    >();
}

/// `payable_any_1`'s return type. `EgldOrEsdtTokenIdentifier<M>` does have a dedicated marker
/// (`EgldOrEsdtTokenIdentifierAbi`), same as `BigUint<M>` -> `BigUintAbi`, so both members of the
/// tuple collapse and the projection is fully API-parameter-free.
#[test]
fn abi_projection_multi_value_2_biguint_egld_or_esdt_token_identifier() {
    assert_type_eq::<
        <MultiValue2<BigUint<Api>, EgldOrEsdtTokenIdentifier<Api>> as TypeAbi>::Abi,
        MultiValue2<BigUintAbi, EgldOrEsdtTokenIdentifierAbi>,
    >();
}

/// `payable_legacy_egld_esdt`'s return type: a mix of both behaviors in one signature - the
/// `BigUint` member collapses to `BigUintAbi`, while the nested `ManagedVec<EsdtTokenPayment>`
/// keeps its managed API parameter (fixed at `UncallableApi`), exactly as in
/// `abi_projection_managed_vec_of_esdt_token_payment` above.
#[test]
fn abi_projection_multi_value_2_biguint_managed_vec_esdt_token_payment() {
    assert_type_eq::<
        <MultiValue2<BigUint<Api>, ManagedVec<Api, EsdtTokenPayment<Api>>> as TypeAbi>::Abi,
        MultiValue2<BigUintAbi, ListAbi<EsdtTokenPayment<Api>>>,
    >();
}
