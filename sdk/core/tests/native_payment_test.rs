use multiversx_sc::types::{BigUint, EsdtTokenPayment, TokenIdentifier};
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::NativeConvertible;
use multiversx_sdk::data::types::payment::Payment;

#[test]
fn test_esdt_token_payment_to_native() {
    let _ = DebugApi::dummy();
    let token_identifier: TokenIdentifier<DebugApi> = TokenIdentifier::from("WEGLD-abcdef");
    let token_nonce = 14u64;
    let amount = BigUint::from(100u64);

    let esdt_token_payment = EsdtTokenPayment::new(
        token_identifier,
        token_nonce,
        amount
    );
    let native = esdt_token_payment.to_native();

    let expected_result = Payment {
        token_identifier: "WEGLD-abcdef".to_string(),
        token_nonce,
        amount: num_bigint::BigUint::from(100u64),
    };

    assert_eq!(
        native,
        expected_result
    );
}