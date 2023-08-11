use multiversx_sc::types::{
    BoxedBytes, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenPayment, ManagedBuffer,
    TokenIdentifier,
};
use multiversx_sc_scenario::{
    api::StaticApi, managed_egld_token_id, managed_test_util::check_managed_top_encode_decode,
    managed_token_id, managed_token_id_wrapped, multiversx_sc,
};

#[test]
fn test_egld() {
    assert!(EgldOrEsdtTokenIdentifier::<StaticApi>::egld().is_egld());
}

#[test]
fn test_codec() {
    check_managed_top_encode_decode(
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld(),
        EgldOrEsdtTokenIdentifier::<StaticApi>::EGLD_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &EgldOrEsdtTokenIdentifier::<StaticApi>::EGLD_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        vec![EgldOrEsdtTokenIdentifier::<StaticApi>::egld()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_esdt_identifier() {
    // valid identifier
    assert!(TokenIdentifier::<StaticApi>::from("ALC-6258d2").is_valid_esdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<StaticApi>::from("ALC123-6258d2").is_valid_esdt_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<StaticApi>::from("12345-6258d2").is_valid_esdt_identifier());

    // missing dash
    assert!(!TokenIdentifier::<StaticApi>::from("ALC6258d2").is_valid_esdt_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<StaticApi>::from("AL-C6258d2").is_valid_esdt_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<StaticApi>::from("alc-6258d2").is_valid_esdt_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258D2").is_valid_esdt_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258d2ff").is_valid_esdt_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<StaticApi>::from("AL-6258d2").is_valid_esdt_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").is_valid_esdt_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC123-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("12345-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-C6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("alc-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258D2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // too many random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258d2ff").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC-6"),
    );

    // ticker too short
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_egld_or_esdt() {
    // egld is always valid
    assert!(EgldOrEsdtTokenIdentifier::<StaticApi>::egld().is_valid());

    // valid esdt
    assert!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenIdentifier::from("ALC-6258d2"))
            .is_valid()
    );

    // invalid esdt, see above
    assert!(
        !EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ESDT-00000"),
        TokenIdentifier::<StaticApi>::from("ESDT-00000")
    );
    assert_ne!(
        TokenIdentifier::<StaticApi>::from("ESDT-00001"),
        TokenIdentifier::<StaticApi>::from("ESDT-00002")
    );

    assert_eq!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenIdentifier::from("ESDT-00003")),
        TokenIdentifier::<StaticApi>::from("ESDT-00003")
    );
    assert_ne!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld(),
        TokenIdentifier::<StaticApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld(),
        TokenIdentifier::<StaticApi>::from("EGLD")
    );
}

#[test]
fn test_payment_eq() {
    assert_eq!(
        EsdtTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        EsdtTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        EsdtTokenPayment::<StaticApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        EsdtTokenPayment::<StaticApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        EgldOrEsdtTokenPayment::<StaticApi>::no_payment(),
        EgldOrEsdtTokenPayment::<StaticApi>::no_payment(),
    );
    assert_eq!(
        EgldOrEsdtTokenPayment::<StaticApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00000"),
            0,
            1000u32.into()
        ),
        EgldOrEsdtTokenPayment::<StaticApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        EgldOrEsdtTokenPayment::<StaticApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00001"),
            0,
            1000u32.into()
        ),
        EgldOrEsdtTokenPayment::<StaticApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        EgldOrEsdtTokenPayment::<StaticApi>::new(
            EgldOrEsdtTokenIdentifier::esdt("ESDTPAY-00001"),
            0,
            1000u32.into()
        ),
        EgldOrEsdtTokenPayment::<StaticApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    assert_eq!(
        managed_egld_token_id!(),
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<StaticApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_esdt(),
        TokenIdentifier::<StaticApi>::from("ALC-6258d2")
    )
}
