use super::RawHandle;

/// Used as a flag. Reading from this handle will always result in a crash.
///
/// Do not initialize!
pub const UNINITIALIZED_HANDLE: RawHandle = i32::MAX;

/// WARNING! With the current VM this still needs to be initialized before use.
pub const BIG_INT_CONST_ZERO: RawHandle = -10;
pub const BIG_INT_TEMPORARY_1: RawHandle = -11;
pub const BIG_INT_TEMPORARY_2: RawHandle = -12;
pub const BIG_FLOAT_TEMPORARY: RawHandle = -15;

/// WARNING! With the current VM this still needs to be initialized before use.
pub const MBUF_CONST_EMPTY: RawHandle = -20;
pub const MBUF_TEMPORARY_1: RawHandle = -25;
pub const MBUF_TEMPORARY_2: RawHandle = -26;

pub const ADDRESS_CALLER: RawHandle = -30;
pub const ADDRESS_SELF: RawHandle = -31;

pub const CALL_VALUE_EGLD: RawHandle = -35;
pub const CALL_VALUE_EGLD_MULTI: RawHandle = -36;
pub const CALL_VALUE_EGLD_FROM_ESDT: RawHandle = -37;
pub const CALL_VALUE_MULTI_ESDT: RawHandle = -38;
pub const CALL_VALUE_ALL: RawHandle = -39;
pub const MBUF_EGLD_000000: RawHandle = -40;
pub const PAYMENTS_SINGLETON_TEMPORARY: RawHandle = -41;

pub const CALLBACK_CLOSURE_ARGS_BUFFER: RawHandle = -50;

pub const NEW_HANDLE_START_FROM: RawHandle = -200; // > -100 reserved for APIs

// Vec of 64 entries of 1 bit
pub const SCALING_FACTOR_START: RawHandle = -100;
pub const SCALING_FACTOR_LENGTH: usize = 64;

/// Used as a flag. Do not use as a regular handle.
pub const MANAGED_OPTION_NONE: RawHandle = i32::MAX - 1;

pub const fn get_scaling_factor_handle(decimals: usize) -> i32 {
    let decimals_i32 = decimals as i32;
    SCALING_FACTOR_START - decimals_i32
}

/// Payload of the singleton ManagedVec that contains the current single EGLD transfer, modelled as an ESDT payment.
pub const EGLD_PAYMENT_PAYLOAD: [u8; 16] = [
    0xff,
    0xff,
    0xff,
    (0x0100 + MBUF_EGLD_000000) as u8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0xff,
    0xff,
    0xff,
    (0x0100 + CALL_VALUE_EGLD) as u8,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egld_payment_payload_test() {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&EGLD_PAYMENT_PAYLOAD[0..4]);
        assert_eq!(i32::from_be_bytes(bytes), MBUF_EGLD_000000);
        bytes.copy_from_slice(&EGLD_PAYMENT_PAYLOAD[12..16]);
        assert_eq!(i32::from_be_bytes(bytes), CALL_VALUE_EGLD);
    }
}
