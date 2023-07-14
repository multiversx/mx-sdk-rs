#[macro_export]
macro_rules! rust_biguint {
    ($value:expr) => {{
        multiversx_sc_scenario::num_bigint::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_biguint {
    ($value:expr) => {{
        multiversx_sc::types::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_buffer {
    ($value:expr) => {{
        multiversx_sc::types::ManagedBuffer::new_from_bytes($value)
    }};
}

#[macro_export]
macro_rules! managed_address {
    ($address:expr) => {{
        multiversx_sc::types::ManagedAddress::from_address($address)
    }};
}

#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        multiversx_sc::types::TokenIdentifier::from_esdt_bytes($bytes)
    }};
}

#[macro_export]
macro_rules! managed_token_id_wrapped {
    ($bytes:expr) => {{
        let ___esdt_token_id___ = multiversx_sc::types::TokenIdentifier::from_esdt_bytes($bytes);
        multiversx_sc::types::EgldOrEsdtTokenIdentifier::esdt(___esdt_token_id___)
    }};
}

#[macro_export]
macro_rules! managed_egld_token_id {
    () => {{
        multiversx_sc::types::EgldOrEsdtTokenIdentifier::egld()
    }};
}

#[macro_export]
macro_rules! assert_sc_error {
    ($sc_result:expr, $expected_string:expr) => {{
        match $sc_result {
            multiversx_sc::types::SCResult::Ok(t) => {
                panic!("Expected SCError, but got SCResult::Ok: {:?}", t)
            },
            multiversx_sc::types::SCResult::Err(err) => {
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
            multiversx_sc::types::SCResult::Ok(t) => t,
            multiversx_sc::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                panic!("{}", as_str);
            },
        }
    }};
}
