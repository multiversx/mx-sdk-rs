use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, reconstruct_trait::ReconstructorContext,
    serde_raw::ValueSubTree, value_interpreter::ExprReconstructorHint,
};

#[test]
fn test_string() {
    let mut interpreted = InterpreterContext::builder().interpret_string("``abcdefg");
    assert_eq!(
        ValueSubTree::Str("str:abcdefg".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("``");
    assert_eq!(
        ValueSubTree::Str("str:".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("```");
    assert_eq!(
        ValueSubTree::Str("str:`".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("`` ");
    assert_eq!(
        ValueSubTree::Str("str: ".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("''abcdefg");
    assert_eq!(
        ValueSubTree::Str("str:abcdefg".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("''");
    assert_eq!(
        ValueSubTree::Str("str:".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("'''");
    assert_eq!(
        ValueSubTree::Str("str:'".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("'' ");
    assert_eq!(
        ValueSubTree::Str("str: ".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("''``");
    assert_eq!(
        ValueSubTree::Str("str:``".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("``''");
    assert_eq!(
        ValueSubTree::Str("str:''".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("str:abcdefg");
    assert_eq!(
        ValueSubTree::Str("str:abcdefg".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("str:");
    assert_eq!(
        ValueSubTree::Str("str:".to_string()),
        ReconstructorContext::builder().reconstruct(&interpreted, &ExprReconstructorHint::StrHint),
    );
}

#[test]
fn test_address() {
    let mut interpreted = InterpreterContext::builder().interpret_string("address:");
    assert_eq!(
        ValueSubTree::Str("address:".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("address:a");
    assert_eq!(
        ValueSubTree::Str("address:a".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("address:a\x05");
    assert_eq!(
        ValueSubTree::Str("address:a\x05".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("address:an_address");
    assert_eq!(
        ValueSubTree::Str("address:an_address".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder()
        .interpret_string("address:1234567890123456789012345678901\x01");
    assert_eq!(
        ValueSubTree::Str("0x3132333435363738393031323334353637383930313233343536373839303101 (address:1234567890123456789012345678901#01)".to_string()),
        ReconstructorContext::builder().reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint
        ),
    );

    // trims excess
    interpreted = InterpreterContext::builder()
        .interpret_string("address:1234567890123456789012345678901\x013");
    assert_eq!(
        ValueSubTree::Str("0x3132333435363738393031323334353637383930313233343536373839303101 (address:1234567890123456789012345678901#01)".to_string()),
        ReconstructorContext::builder().reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint
        ),
    );
}

#[test]
fn test_address_with_shard_id() {
    let mut interpreted = InterpreterContext::builder().interpret_string("address:#05");
    assert_eq!(
        ValueSubTree::Str(
            "0x5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f05 (address:#05)"
                .to_string()
        ),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("address:a#bb");
    assert_eq!(
        ValueSubTree::Str(
            "0x615f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5fbb (address:a#bb)"
                .to_string()
        ),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("address:an_address#99");
    assert_eq!(
        ValueSubTree::Str("0x616e5f616464726573735f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f99 (address:an_address#99)".to_string()),
        ReconstructorContext::builder().reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint
        ),
    );

    interpreted = InterpreterContext::builder()
        .interpret_string("address:1234567890123456789012345678901#66");
    assert_eq!(
        ValueSubTree::Str("address:1234567890123456789012345678901#66".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    // trims excess
    interpreted = InterpreterContext::builder()
        .interpret_string("address:12345678901234567890123456789012#66");
    assert_eq!(
        ValueSubTree::Str("address:1234567890123456789012345678901#66".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );
}

#[test]
fn test_scaddress() {
    let mut interpreted = InterpreterContext::builder().interpret_string("sc:a");
    assert_eq!(
        ValueSubTree::Str("sc:a".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("sc:123456789012345678912s");
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#73".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    // trims excess
    interpreted = InterpreterContext::builder().interpret_string("sc:123456789012345678912sx");
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#73".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );
}

#[test]
fn test_scaddress_with_shard_id() {
    let mut interpreted = InterpreterContext::builder().interpret_string("sc:a#44");
    assert_eq!(
        ValueSubTree::Str("sc:a#44".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("sc:123456789012345678912#88");
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#88".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );

    // trims excess
    interpreted = InterpreterContext::builder().interpret_string("sc:123456789012345678912x#88");
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#88".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::AddressHint),
    );
}

#[test]
fn test_unsigned_number() {
    let mut interpreted = InterpreterContext::builder().interpret_string("0x");
    assert_eq!(
        ValueSubTree::Str("0".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("0");
    assert_eq!(
        ValueSubTree::Str("0".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("12");
    assert_eq!(
        ValueSubTree::Str("12".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("256");
    assert_eq!(
        ValueSubTree::Str("256".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("0b1");
    assert_eq!(
        ValueSubTree::Str("1".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );
}

#[test]
fn test_signed_number() {
    let mut interpreted = InterpreterContext::builder().interpret_string("255");
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("+255");
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("0xff");
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );

    interpreted = InterpreterContext::builder().interpret_string("+0xff");
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        ReconstructorContext::builder()
            .reconstruct(&interpreted, &ExprReconstructorHint::UnsignedNumberHint),
    );
}
