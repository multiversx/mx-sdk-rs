#[macro_export]
macro_rules! rust_biguint {
    ($value:expr) => {{
        num_bigint::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_biguint {
    ($value:expr) => {{
        BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_address {
    ($address:expr) => {{
        ManagedAddress::from_address($address)
    }};
}

#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        TokenIdentifier::from_esdt_bytes($bytes)
    }};
}

#[macro_export]
macro_rules! assert_sc_error {
    ($sc_result:expr, $expected_string:expr) => {{
        assert_eq!($sc_result.err().unwrap().as_bytes(), $expected_string)
    }};
}
