use core::num::NonZeroU64;
use multiversx_sc::{
    chain_core::EGLD_000000_TOKEN_IDENTIFIER,
    types::{BigUint, EgldOrEsdtTokenIdentifier, NonZeroBigUint, NonZeroError, Payment},
};
use multiversx_sc_scenario::api::StaticApi;
use std::convert::Infallible;

type SA = StaticApi;

const TOKEN_ID: &str = "MYTOKEN-123456";

#[test]
fn test_payment_try_new_error() {
    // u32
    assert_eq!(Payment::<SA>::try_new(TOKEN_ID, 0, 0u32), Err(NonZeroError));

    // u64 amount
    assert_eq!(Payment::<SA>::try_new(TOKEN_ID, 0, 0u64), Err(NonZeroError));

    // BigUint amount
    assert_eq!(
        Payment::<SA>::try_new(TOKEN_ID, 0, BigUint::<SA>::zero()),
        Err(NonZeroError)
    );
}

#[test]
fn test_payment_try_new_various_types() {
    // u32 amount
    let payment_u32 = Payment::<SA>::try_new(TOKEN_ID, 0, 42u32).unwrap();
    assert_eq!(payment_u32.amount, 42u32);

    // u64 amount
    let payment_u64 = Payment::<SA>::try_new(TOKEN_ID, 0, 123u64).unwrap();
    assert_eq!(payment_u64.amount, 123u64);

    // NonZeroU64 amount
    let nz_u64 = NonZeroU64::new(77u64).unwrap();
    let result: Result<Payment<SA>, Infallible> = Payment::<SA>::try_new(TOKEN_ID, 0, nz_u64);
    let Ok(payment_nz_u64) = result; // infallible
    assert_eq!(payment_nz_u64.amount, 77u64);

    // BigUint amount
    let biguint = BigUint::<SA>::from(999u64);
    let payment_bu = Payment::<SA>::try_new(TOKEN_ID, 0, biguint.clone()).unwrap();
    assert_eq!(payment_bu.amount, biguint);

    // NonZeroBigUint amount
    let nz_biguint = NonZeroBigUint::<SA>::try_from(555u64).unwrap();
    let result: Result<Payment<SA>, Infallible> =
        Payment::<SA>::try_new(TOKEN_ID, 0, nz_biguint.clone());
    let Ok(payment_nzbu) = result; // infallible
    assert_eq!(payment_nzbu.amount, nz_biguint);
}

#[test]
fn test_payment_to_egld_or_esdt_token_payment_conversions() {
    type SA = StaticApi;
    const TOKEN_ID_STR: &str = "MYTOKEN-123456";
    let token_id = EgldOrEsdtTokenIdentifier::<SA>::from(TOKEN_ID_STR);
    let nonce = 7u64;
    let amount = 12345u64;

    // Start from Payment object (ESDT)
    let payment_esdt = Payment::<SA>::try_new(TOKEN_ID_STR, nonce, amount).unwrap();

    // as_egld_or_esdt_payment (via Payment -> EsdtTokenPayment)
    let egld_or_esdt_ref = payment_esdt.as_egld_or_esdt_payment();
    assert!(egld_or_esdt_ref.token_identifier.is_esdt());
    assert_eq!(egld_or_esdt_ref.token_identifier, token_id.clone());
    assert_eq!(egld_or_esdt_ref.token_nonce, nonce);
    assert_eq!(egld_or_esdt_ref.amount, amount);

    // into_egld_or_esdt_payment (via Payment)
    let egld_or_esdt = payment_esdt.clone().into_egld_or_esdt_payment();
    assert!(egld_or_esdt.token_identifier.is_esdt());
    assert_eq!(egld_or_esdt.token_identifier, token_id.clone());
    assert_eq!(egld_or_esdt.token_nonce, nonce);
    assert_eq!(egld_or_esdt.amount, amount);

    // Example with EGLD
    let egld_amount = 54321u64;
    let payment_egld =
        Payment::<SA>::try_new(EGLD_000000_TOKEN_IDENTIFIER, 0, egld_amount).unwrap();

    // as_egld_or_esdt_payment (via Payment -> EGLD)
    let egld_ref = payment_egld.as_egld_or_esdt_payment();
    assert!(egld_ref.token_identifier.is_egld());
    assert_eq!(
        egld_ref.token_identifier,
        EgldOrEsdtTokenIdentifier::<SA>::egld()
    );
    assert_eq!(egld_ref.token_nonce, 0);
    assert_eq!(egld_ref.amount, egld_amount);

    // into_egld_or_esdt_payment (via Payment)
    let egld = payment_egld.clone().into_egld_or_esdt_payment();
    assert!(egld.token_identifier.is_egld());
    assert_eq!(
        egld.token_identifier,
        EgldOrEsdtTokenIdentifier::<SA>::egld()
    );
    assert_eq!(egld.token_nonce, 0);
    assert_eq!(egld.amount, egld_amount);
}
