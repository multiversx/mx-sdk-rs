use multiversx_sc::{
    chain_core::EGLD_000000_TOKEN_IDENTIFIER,
    types::{EgldOrEsdtTokenIdentifier, Payment},
};
use multiversx_sc_scenario::api::StaticApi;

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
