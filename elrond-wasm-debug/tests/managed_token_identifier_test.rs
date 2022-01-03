use elrond_wasm::types::{BoxedBytes, TokenIdentifier};
use elrond_wasm_debug::{check_managed_top_decode, check_managed_top_encode_decode, DebugApi};

#[test]
fn test_egld() {
    let _ = DebugApi::dummy();
    assert!(TokenIdentifier::<DebugApi>::egld().is_egld());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        TokenIdentifier::<DebugApi>::egld(),
        TokenIdentifier::<DebugApi>::EGLD_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &TokenIdentifier::<DebugApi>::EGLD_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api.clone(),
        vec![TokenIdentifier::<DebugApi>::egld()],
        expected.as_slice(),
    );

    // also allowed
    assert_eq!(
        TokenIdentifier::<DebugApi>::egld(),
        check_managed_top_decode::<TokenIdentifier<DebugApi>>(api.clone(), &[])
    );
    assert_eq!(
        vec![TokenIdentifier::<DebugApi>::egld()],
        check_managed_top_decode::<Vec<TokenIdentifier<DebugApi>>>(api, &[0, 0, 0, 0])
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_esdt_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"ALC-6258d2"[..]).is_valid_esdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"ALC123-6258d2"[..]).is_valid_esdt_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"12345-6258d2"[..]).is_valid_esdt_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"ALC6258d2"[..]).is_valid_esdt_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"AL-C6258d2"[..]).is_valid_esdt_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"alc-6258d2"[..]).is_valid_esdt_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"ALC-6258D2"[..]).is_valid_esdt_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"ALC-6258d2ff"[..]).is_valid_esdt_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"AL-6258d2"[..]).is_valid_esdt_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from_esdt_bytes(&b"ALCCCCCCCCC-6258d2"[..]).is_valid_esdt_identifier());
}
