use elrond_wasm::types::{BoxedBytes, TokenIdentifier};
use elrond_wasm_debug::{check_managed_top_decode, check_managed_top_encode_decode, DebugApi};

#[test]
fn test_egld() {
    let api = DebugApi::dummy();
    assert!(TokenIdentifier::egld(api).is_egld());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        TokenIdentifier::egld(api.clone()),
        TokenIdentifier::<DebugApi>::EGLD_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &TokenIdentifier::<DebugApi>::EGLD_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api.clone(),
        vec![TokenIdentifier::egld(api.clone())],
        expected.as_slice(),
    );

    // also allowed
    assert_eq!(
        TokenIdentifier::egld(api.clone()),
        check_managed_top_decode::<TokenIdentifier<DebugApi>>(api.clone(), &[])
    );
    assert_eq!(
        vec![TokenIdentifier::egld(api.clone())],
        check_managed_top_decode::<Vec<TokenIdentifier<DebugApi>>>(api, &[0, 0, 0, 0])
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_esdt_identifier() {
    let api = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::from_esdt_bytes(api.clone(), &b"ALC-6258d2"[..]).is_valid_esdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::from_esdt_bytes(api.clone(), &b"ALC123-6258d2"[..]).is_valid_esdt_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::from_esdt_bytes(api.clone(), &b"12345-6258d2"[..]).is_valid_esdt_identifier());

    // missing dash
    assert!(!TokenIdentifier::from_esdt_bytes(api.clone(), &b"ALC6258d2"[..]).is_valid_esdt_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::from_esdt_bytes(api.clone(), &b"AL-C6258d2"[..]).is_valid_esdt_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::from_esdt_bytes(api.clone(), &b"alc-6258d2"[..]).is_valid_esdt_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::from_esdt_bytes(api.clone(), &b"ALC-6258D2"[..]).is_valid_esdt_identifier());

    // too many random chars
    assert!(!TokenIdentifier::from_esdt_bytes(api.clone(), &b"ALC-6258d2ff"[..]).is_valid_esdt_identifier());

    // ticker too short
    assert!(!TokenIdentifier::from_esdt_bytes(api.clone(), &b"AL-6258d2"[..]).is_valid_esdt_identifier());

    // ticker too long
    assert!(!TokenIdentifier::from_esdt_bytes(api, &b"ALCCCCCCCCC-6258d2"[..]).is_valid_esdt_identifier());
}
