use multiversx_sc::types::{
    BoxedBytes, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenPayment,
    TokenIdentifier,
};
use multiversx_sc_scenario::{
    managed_egld_token_id, managed_token_id, managed_token_id_wrapped,
    multiversx_chain_vm::check_managed_top_encode_decode, multiversx_sc, DebugApi,
};

#[test]
fn test_egld() {
    let _ = DebugApi::dummy();
    assert!(EgldOrEsdtTokenIdentifier::<DebugApi>::egld().is_egld());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        EgldOrEsdtTokenIdentifier::<DebugApi>::egld(),
        EgldOrEsdtTokenIdentifier::<DebugApi>::EGLD_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &EgldOrEsdtTokenIdentifier::<DebugApi>::EGLD_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api,
        vec![EgldOrEsdtTokenIdentifier::<DebugApi>::egld()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_esdt_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from("ALC-6258d2").is_valid_esdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from("ALC123-6258d2").is_valid_esdt_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from("12345-6258d2").is_valid_esdt_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from("ALC6258d2").is_valid_esdt_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from("AL-C6258d2").is_valid_esdt_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from("alc-6258d2").is_valid_esdt_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258D2").is_valid_esdt_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258d2ff").is_valid_esdt_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from("AL-6258d2").is_valid_esdt_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from("ALCCCCCCCCC-6258d2").is_valid_esdt_identifier());
}

#[test]
fn test_is_valid_egld_or_esdt() {
    let _ = DebugApi::dummy();

    // egld is always valid
    assert!(EgldOrEsdtTokenIdentifier::<DebugApi>::egld().is_valid());

    // valid esdt
    assert!(
        EgldOrEsdtTokenIdentifier::<DebugApi>::esdt(TokenIdentifier::from("ALC-6258d2")).is_valid()
    );

    // invalid esdt, see above
    assert!(
        !EgldOrEsdtTokenIdentifier::<DebugApi>::esdt(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        TokenIdentifier::<DebugApi>::from("ESDT-00000"),
        TokenIdentifier::<DebugApi>::from("ESDT-00000")
    );
    assert_ne!(
        TokenIdentifier::<DebugApi>::from("ESDT-00001"),
        TokenIdentifier::<DebugApi>::from("ESDT-00002")
    );

    assert_eq!(
        EgldOrEsdtTokenIdentifier::<DebugApi>::esdt(TokenIdentifier::from("ESDT-00003")),
        TokenIdentifier::<DebugApi>::from("ESDT-00003")
    );
    assert_ne!(
        EgldOrEsdtTokenIdentifier::<DebugApi>::egld(),
        TokenIdentifier::<DebugApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        EgldOrEsdtTokenIdentifier::<DebugApi>::egld(),
        TokenIdentifier::<DebugApi>::from("EGLD")
    );
}

#[test]
fn test_payment_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        EsdtTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        EsdtTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        EsdtTokenPayment::<DebugApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        EsdtTokenPayment::<DebugApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        EgldOrEsdtTokenPayment::<DebugApi>::no_payment(),
        EgldOrEsdtTokenPayment::<DebugApi>::no_payment(),
    );
    assert_eq!(
        EgldOrEsdtTokenPayment::<DebugApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00000"),
            0,
            1000u32.into()
        ),
        EgldOrEsdtTokenPayment::<DebugApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        EgldOrEsdtTokenPayment::<DebugApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00001"),
            0,
            1000u32.into()
        ),
        EgldOrEsdtTokenPayment::<DebugApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        EgldOrEsdtTokenPayment::<DebugApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00001"),
            0,
            1000u32.into()
        ),
        EgldOrEsdtTokenPayment::<DebugApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    let _ = DebugApi::dummy();
    assert_eq!(
        managed_egld_token_id!(),
        EgldOrEsdtTokenIdentifier::<DebugApi>::egld()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_esdt(),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    )
}
