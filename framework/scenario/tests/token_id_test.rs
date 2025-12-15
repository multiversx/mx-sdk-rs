use multiversx_sc::{
    chain_core::EGLD_000000_TOKEN_IDENTIFIER,
    types::{
        BoxedBytes, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenIdentifier,
        EsdtTokenPayment, ManagedBuffer, TokenId,
    },
};
use multiversx_sc_scenario::{
    api::StaticApi, managed_test_util::check_managed_top_encode_decode, multiversx_sc, token_id,
};

#[test]
fn test_codec_top() {
    check_managed_top_encode_decode(
        TokenId::<StaticApi>::from(EGLD_000000_TOKEN_IDENTIFIER),
        EGLD_000000_TOKEN_IDENTIFIER.as_bytes(),
    );
}

#[test]
fn test_codec_nested() {
    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, EGLD_000000_TOKEN_IDENTIFIER.len() as u8],
        EGLD_000000_TOKEN_IDENTIFIER.as_bytes(),
    ]);
    check_managed_top_encode_decode(
        vec![TokenId::<StaticApi>::from(EGLD_000000_TOKEN_IDENTIFIER)],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_esdt_identifier() {
    // valid identifier
    assert!(TokenId::<StaticApi>::from("ALC-6258d2").is_valid_esdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenId::<StaticApi>::from("ALC123-6258d2").is_valid_esdt_identifier());

    // valid ticker only numbers
    assert!(TokenId::<StaticApi>::from("12345-6258d2").is_valid_esdt_identifier());

    // missing dash
    assert!(!TokenId::<StaticApi>::from("ALC6258d2").is_valid_esdt_identifier());

    // wrong dash position
    assert!(!TokenId::<StaticApi>::from("AL-C6258d2").is_valid_esdt_identifier());

    // lowercase ticker
    assert!(!TokenId::<StaticApi>::from("alc-6258d2").is_valid_esdt_identifier());

    // uppercase random chars
    assert!(!TokenId::<StaticApi>::from("ALC-6258D2").is_valid_esdt_identifier());

    // too many random chars
    assert!(!TokenId::<StaticApi>::from("ALC-6258d2ff").is_valid_esdt_identifier());

    // ticker too short
    assert!(!TokenId::<StaticApi>::from("AL-6258d2").is_valid_esdt_identifier());

    // ticker too long
    assert!(!TokenId::<StaticApi>::from("ALCCCCCCCCC-6258d2").is_valid_esdt_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        TokenId::<StaticApi>::from("ALC123-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        TokenId::<StaticApi>::from("12345-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        TokenId::<StaticApi>::from("ALC6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        TokenId::<StaticApi>::from("AL-C6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        TokenId::<StaticApi>::from("alc-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258D2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // too many random chars
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258d2ff").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC-6"),
    );

    // ticker too short
    assert_eq!(
        TokenId::<StaticApi>::from("AL-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        TokenId::<StaticApi>::from("ALCCCCCCCCC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_egld_or_esdt() {
    // egld is always valid
    assert!(EgldOrEsdtTokenIdentifier::<StaticApi>::egld().is_valid());

    // valid esdt
    assert!(EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenId::from("ALC-6258d2")).is_valid());

    // invalid esdt, see above
    assert!(
        !EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenId::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    assert_eq!(
        TokenId::<StaticApi>::from("ESDT-00000"),
        TokenId::<StaticApi>::from("ESDT-00000")
    );
    assert_ne!(
        TokenId::<StaticApi>::from("ESDT-00001"),
        TokenId::<StaticApi>::from("ESDT-00002")
    );

    assert_eq!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenId::from("ESDT-00003")),
        TokenId::<StaticApi>::from("ESDT-00003").into_legacy()
    );
    assert_ne!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld(),
        TokenId::<StaticApi>::from("ANYTHING-1234").into_legacy()
    );
    assert_ne!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld(),
        TokenId::<StaticApi>::from("EGLD").into_legacy()
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
        token_id!(b"ALC-6258d2"),
        TokenId::<StaticApi>::from("ALC-6258d2")
    );
}

#[test]
fn test_token_id_to_string() {
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258d2").to_string(),
        "ALC-6258d2"
    );
    assert_eq!(
        TokenId::<StaticApi>::from("EGLD-00000").to_string(),
        "EGLD-00000"
    );
    assert_eq!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld().to_string(),
        "EGLD"
    );
    assert_eq!(
        EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenId::from("EGLDORESDT-00001")).to_string(),
        "EGLDORESDT-00001"
    );
    assert_eq!(
        EsdtTokenIdentifier::<StaticApi>::from_esdt_bytes("ESDT-00001").to_string(),
        "ESDT-00001"
    );
}
