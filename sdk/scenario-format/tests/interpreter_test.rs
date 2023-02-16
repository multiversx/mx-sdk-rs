use multiversx_chain_scenario_format::interpret_trait::InterpreterContext;

const EMPTY: Vec<u8> = Vec::<u8>::new();

#[test]
fn test_bool() {
    assert_eq!(
        vec![1],
        InterpreterContext::builder().interpret_string("true")
    );
    assert_eq!(
        EMPTY,
        InterpreterContext::builder().interpret_string("false")
    );
}

#[test]
fn test_string() {
    assert_eq!(
        b"abcdefg".to_vec(),
        InterpreterContext::builder().interpret_string("``abcdefg")
    );
    assert_eq!(EMPTY, InterpreterContext::builder().interpret_string("``"));
    assert_eq!(
        b"`".to_vec(),
        InterpreterContext::builder().interpret_string("```")
    );
    assert_eq!(
        b" ".to_vec(),
        InterpreterContext::builder().interpret_string("`` ")
    );

    assert_eq!(
        b"abcdefg".to_vec(),
        InterpreterContext::builder().interpret_string("''abcdefg")
    );
    assert_eq!(EMPTY, InterpreterContext::builder().interpret_string("''"));
    assert_eq!(
        b"'".to_vec(),
        InterpreterContext::builder().interpret_string("'''")
    );
    assert_eq!(
        b"``".to_vec(),
        InterpreterContext::builder().interpret_string("''``")
    );

    assert_eq!(
        b"abcdefg".to_vec(),
        InterpreterContext::builder().interpret_string("str:abcdefg")
    );
    assert_eq!(
        EMPTY,
        InterpreterContext::builder().interpret_string("str:")
    );
}

#[test]
fn test_address() {
    assert_eq!(
        b"________________________________".to_vec(),
        InterpreterContext::builder().interpret_string("address:")
    );
    assert_eq!(
        b"a_______________________________".to_vec(),
        InterpreterContext::builder().interpret_string("address:a")
    );
    assert_eq!(
        b"a\x05______________________________".to_vec(),
        InterpreterContext::builder().interpret_string("address:a\x05")
    );
    assert_eq!(
        b"an_address______________________".to_vec(),
        InterpreterContext::builder().interpret_string("address:an_address")
    );
    assert_eq!(
        b"12345678901234567890123456789012".to_vec(),
        InterpreterContext::builder().interpret_string("address:12345678901234567890123456789012")
    );
    assert_eq!(
        b"12345678901234567890123456789012".to_vec(),
        InterpreterContext::builder().interpret_string("address:123456789012345678901234567890123")
    );
}

#[test]
fn test_address_with_shard_id() {
    assert_eq!(
        b"_______________________________\x05".to_vec(),
        InterpreterContext::builder().interpret_string("address:#05")
    );
    assert_eq!(
        b"a______________________________\xbb".to_vec(),
        InterpreterContext::builder().interpret_string("address:a#bb")
    );
    assert_eq!(
        b"an_address_____________________\x99".to_vec(),
        InterpreterContext::builder().interpret_string("address:an_address#99")
    );
    assert_eq!(
        b"1234567890123456789012345678901\x66".to_vec(),
        InterpreterContext::builder()
            .interpret_string("address:1234567890123456789012345678901#66")
    );
    assert_eq!(
        b"1234567890123456789012345678901\x66".to_vec(),
        InterpreterContext::builder()
            .interpret_string("address:12345678901234567890123456789012#66")
    );
}

#[test]
fn test_sc_address() {
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00a_____________________".to_vec(),
        InterpreterContext::builder().interpret_string("sc:a")
    );
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012".to_vec(),
        InterpreterContext::builder().interpret_string("sc:12345678901234567890120s")
    );
    // trims excess
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012".to_vec(),
        InterpreterContext::builder().interpret_string("sc:12345678901234567890120sx")
    );
}

#[test]
fn test_sc_address_with_shard_id() {
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00a____________________\x44".to_vec(),
        InterpreterContext::builder().interpret_string("sc:a#44")
    );
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00123456789012345678901\x88".to_vec(),
        InterpreterContext::builder().interpret_string("sc:12345678901234567890120#88")
    );
    // trims excess
    assert_eq!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00123456789012345678901\x88".to_vec(),
        InterpreterContext::builder().interpret_string("sc:12345678901234567890120x#88")
    );
}

#[test]
fn test_unsigned_number() {
    assert_eq!(
        vec![0x12, 0x34],
        InterpreterContext::builder().interpret_string("0x1234")
    );
    assert_eq!(
        vec![0x00],
        InterpreterContext::builder().interpret_string("0x0")
    );
    assert_eq!(
        vec![0x00],
        InterpreterContext::builder().interpret_string("0x00")
    );
    assert_eq!(
        vec![0x00, 0x00],
        InterpreterContext::builder().interpret_string("0x000")
    );
    assert_eq!(
        vec![0x00, 0x00],
        InterpreterContext::builder().interpret_string("0x0000")
    );
    assert_eq!(
        vec![0x00, 0x00, 0xab],
        InterpreterContext::builder().interpret_string("0x0000ab")
    );
    assert_eq!(EMPTY, InterpreterContext::builder().interpret_string("0x"));
    assert_eq!(EMPTY, InterpreterContext::builder().interpret_string("0"));
    assert_eq!(
        vec![12],
        InterpreterContext::builder().interpret_string("12")
    );
    assert_eq!(
        vec![0x01, 0x00],
        InterpreterContext::builder().interpret_string("256")
    );
    assert_eq!(
        vec![0x01],
        InterpreterContext::builder().interpret_string("0b1")
    );
    assert_eq!(
        vec![0x05],
        InterpreterContext::builder().interpret_string("0b101")
    );
}

#[test]
fn test_signed_number() {
    assert_eq!(
        vec![0xff],
        InterpreterContext::builder().interpret_string("-1")
    );
    assert_eq!(
        vec![0xff],
        InterpreterContext::builder().interpret_string("255")
    );
    assert_eq!(
        vec![0xff],
        InterpreterContext::builder().interpret_string("0xff")
    );
    assert_eq!(
        vec![0x00, 0xff],
        InterpreterContext::builder().interpret_string("+255")
    );
    assert_eq!(
        vec![0x00, 0xff],
        InterpreterContext::builder().interpret_string("+0xff")
    );

    assert_eq!(
        vec![0xff, 0x00],
        InterpreterContext::builder().interpret_string("-256")
    );
    assert_eq!(
        vec![0xfb],
        InterpreterContext::builder().interpret_string("-0b101")
    );
}

#[test]
fn test_unsigned_fixed_width() {
    assert_eq!(
        vec![0x00],
        InterpreterContext::builder().interpret_string("u8:0")
    );
    assert_eq!(
        vec![0x00, 0x00],
        InterpreterContext::builder().interpret_string("u16:0")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00],
        InterpreterContext::builder().interpret_string("u32:0")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        InterpreterContext::builder().interpret_string("u64:0")
    );
    assert_eq!(
        vec![0x12, 0x34],
        InterpreterContext::builder().interpret_string("u16:0x1234")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x12, 0x34],
        InterpreterContext::builder().interpret_string("u32:0x1234")
    );
    assert_eq!(
        vec![0x01, 0x00],
        InterpreterContext::builder().interpret_string("u16:256")
    );
    assert_eq!(
        vec![0x01],
        InterpreterContext::builder().interpret_string("u8:0b1")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05],
        InterpreterContext::builder().interpret_string("u64:0b101")
    );
}

#[test]
fn test_signed_fixed_width() {
    assert_eq!(
        vec![0x00],
        InterpreterContext::builder().interpret_string("i8:0")
    );
    assert_eq!(
        vec![0x00, 0x00],
        InterpreterContext::builder().interpret_string("i16:0")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00],
        InterpreterContext::builder().interpret_string("i32:0")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        InterpreterContext::builder().interpret_string("i64:0")
    );

    assert_eq!(
        vec![0xff],
        InterpreterContext::builder().interpret_string("i8:-1")
    );
    assert_eq!(
        vec![0xff, 0xff],
        InterpreterContext::builder().interpret_string("i16:-1")
    );
    assert_eq!(
        vec![0xff, 0xff, 0xff, 0xff],
        InterpreterContext::builder().interpret_string("i32:-1")
    );
    assert_eq!(
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        InterpreterContext::builder().interpret_string("i64:-1")
    );
    assert_eq!(
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00],
        InterpreterContext::builder().interpret_string("i64:-256")
    );
    assert_eq!(
        vec![0xfb],
        InterpreterContext::builder().interpret_string("i8:-0b101")
    );
}

#[test]
#[should_panic]
fn test_signed_fixed_width_panic_1() {
    InterpreterContext::builder().interpret_string("i8:+255");
}

#[test]
#[should_panic]
fn test_signed_fixed_width_panic_2() {
    InterpreterContext::builder().interpret_string("i8:0xff");
}

#[test]
#[should_panic]
fn test_signed_fixed_width_panic_3() {
    InterpreterContext::builder().interpret_string("i8:-255");
}

#[test]
fn test_biguint_nested() {
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x00],
        InterpreterContext::builder().interpret_string("biguint:0")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x01, 0x01],
        InterpreterContext::builder().interpret_string("biguint:1")
    );
    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x02, 0x01, 0xFF],
        InterpreterContext::builder().interpret_string("biguint:0x01FF")
    );

    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05],
        InterpreterContext::builder().interpret_string("biguint:0x0102030405")
    );

    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05],
        InterpreterContext::builder().interpret_string("nested:0x0102030405")
    );

    assert_eq!(
        vec![0x00, 0x00, 0x00, 0x01, 0xFF],
        InterpreterContext::builder().interpret_string("nested:-1")
    );
}

#[test]
#[should_panic]
fn test_biguint_nested_neg() {
    InterpreterContext::builder().interpret_string("biguint:-1");
}

#[test]
fn test_bech32() {
    // alice
    assert_eq!(
        hex::decode("0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1").unwrap(),
        InterpreterContext::builder().interpret_string(
            "bech32:erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
        )
    );

    // system SC
    assert_eq!(
        hex::decode("000000000000000000010000000000000000000000000000000000000002ffff").unwrap(),
        InterpreterContext::builder().interpret_string(
            "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
        )
    );
}
