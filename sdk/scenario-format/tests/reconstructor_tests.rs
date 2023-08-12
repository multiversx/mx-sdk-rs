use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext,
    reconstruct_trait::ReconstructorContext,
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, reconstruct, ExprReconstructorHint},
};

#[test]
fn test_string() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("``abcdefg", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:abcdefg".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("``", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("```", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:`".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("`` ", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str: ".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("''abcdefg", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:abcdefg".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("''", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("'''", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:'".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("'' ", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str: ".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("''``", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:``".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("``''", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:''".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("str:abcdefg", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:abcdefg".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("str:", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("str:".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::StrHint,
            &reconstructor_context
        ),
    );
}

#[test]
fn test_address() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("address:", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("address:".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("address:a", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("address:a".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("address:a\x05", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("address:a\x05".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("address:an_address", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("address:an_address".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string(
        "address:1234567890123456789012345678901\x01",
        &interpreter_context,
    );
    assert_eq!(
        ValueSubTree::Str("0x3132333435363738393031323334353637383930313233343536373839303101 (address:1234567890123456789012345678901#01)".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    // trims excess
    interpreted = interpret_string(
        "address:1234567890123456789012345678901\x013",
        &interpreter_context,
    );
    assert_eq!(
        ValueSubTree::Str("0x3132333435363738393031323334353637383930313233343536373839303101 (address:1234567890123456789012345678901#01)".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );
}

#[test]
fn test_address_with_shard_id() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("address:#05", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str(
            "0x5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f05 (address:#05)"
                .to_string()
        ),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("address:a#bb", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str(
            "0x615f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5fbb (address:a#bb)"
                .to_string()
        ),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("address:an_address#99", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("0x616e5f616464726573735f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f99 (address:an_address#99)".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string(
        "address:1234567890123456789012345678901#66",
        &interpreter_context,
    );
    assert_eq!(
        ValueSubTree::Str("address:1234567890123456789012345678901#66".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    // trims excess
    interpreted = interpret_string(
        "address:12345678901234567890123456789012#66",
        &interpreter_context,
    );
    assert_eq!(
        ValueSubTree::Str("address:1234567890123456789012345678901#66".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );
}

#[test]
fn test_scaddress() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("sc:a", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("sc:a".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("sc:123456789012345678912s", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#73".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    // trims excess
    interpreted = interpret_string("sc:123456789012345678912sx", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#73".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );
}

#[test]
fn test_scaddress_with_shard_id() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("sc:a#44", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("sc:a#44".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("sc:123456789012345678912#88", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#88".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );

    // trims excess
    interpreted = interpret_string("sc:123456789012345678912x#88", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("sc:123456789012345678912#88".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::AddressHint,
            &reconstructor_context
        ),
    );
}

#[test]
fn test_unsigned_number() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("0x", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("0".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("0", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("0".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("12", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("12".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("256", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("256".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("0b1", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("1".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );
}

#[test]
fn test_signed_number() {
    let interpreter_context = InterpreterContext::default();
    let reconstructor_context = ReconstructorContext::default();
    let mut interpreted = interpret_string("255", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("+255", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("0xff", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );

    interpreted = interpret_string("+0xff", &interpreter_context);
    assert_eq!(
        ValueSubTree::Str("255".to_string()),
        reconstruct(
            &interpreted,
            &ExprReconstructorHint::UnsignedNumberHint,
            &reconstructor_context
        ),
    );
}
