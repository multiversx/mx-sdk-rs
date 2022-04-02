#[macro_export]
macro_rules! rust_biguint {
    ($value:expr) => {{
        elrond_wasm::num_bigint::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_biguint {
    ($value:expr) => {{
        elrond_wasm::types::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_buffer {
    ($value:expr) => {{
        elrond_wasm::types::ManagedBuffer::new_from_bytes($value)
    }};
}

#[macro_export]
macro_rules! managed_address {
    ($address:expr) => {{
        elrond_wasm::types::ManagedAddress::from_address($address)
    }};
}

#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        elrond_wasm::types::TokenIdentifier::from_esdt_bytes($bytes)
    }};
}

#[macro_export]
macro_rules! assert_sc_error {
    ($sc_result:expr, $expected_string:expr) => {{
        match $sc_result {
            elrond_wasm::types::SCResult::Ok(t) => {
                panic!("Expected SCError, but got SCResult::Ok: {:?}", t)
            },
            elrond_wasm::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                assert_eq!(as_str, $expected_string);
            },
        }
    }};
}

#[macro_export]
macro_rules! assert_values_eq {
    ($left:expr, $right:expr) => {{
        assert!(
            $left == $right,
            "Assert mismatch: \n Left: {:?} \n Right: {:?}",
            $left,
            $right
        )
    }};
}

#[macro_export]
macro_rules! unwrap_or_panic {
    ($sc_result:expr) => {{
        match $sc_result {
            elrond_wasm::types::SCResult::Ok(t) => t,
            elrond_wasm::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                panic!("{}", as_str);
            },
        }
    }};
}
