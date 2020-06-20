
pub const NON_PAYABLE: &[u8] = "attempted to transfer funds via a non-payable function".as_bytes();

pub const ARG_WRONG_NUMBER: &[u8] = "wrong number of arguments".as_bytes();
pub const ARG_ASYNC_RETURN_WRONG_NUMBER: &[u8] = "wrong number of arguments returned by async call".as_bytes();
pub const ARG_CALLBACK_TOO_FEW:  &[u8] = "too few callback arguments provided".as_bytes();
pub const ARG_CALLBACK_TOO_MANY: &[u8] = "too many callback arguments provided".as_bytes();

pub const ARG_OUT_OF_RANGE: &[u8] = "argument out of range".as_bytes();
pub const ARG_BAD_LENGTH: &[u8] = "argument has wrong length".as_bytes();
pub const ARG_BAD_LENGTH_32: &[u8] = "argument has wrong length: 32 as_bytes expected".as_bytes();

pub const BIG_UINT_EXCEEDS_SLICE: &[u8] = "big uint as_bytes exceed target slice".as_bytes();
pub const BIG_UINT_SUB_NEGATIVE: &[u8] = "cannot subtract because result would be negative".as_bytes();

pub const DESERIALIZATION_INVALID_BYTE: &[u8] = "call data deserialization error: not a valid byte".as_bytes();
pub const DESERIALIZATION_NOT_32_BYTES: &[u8] = "call data deserialization error: 32 as_bytes expected".as_bytes();
pub const DESERIALIZATION_ODD_DIGITS: &[u8] = "call data deserialization error: odd number of digits in hex representation".as_bytes();
pub const DESERIALIZATION_ARG_OUT_OF_RANGE: &[u8] = "call data deserialization error: argument out of range".as_bytes();

pub const CALLBACK_NONE: &[u8] = "no callbacks in contract".as_bytes();
pub const CALLBACK_BAD_FUNC: &[u8] = "no callback function with that name exists in contract".as_bytes();

pub const STORAGE_NOT_I64: &[u8] = "storage not i64".as_bytes();
pub const STORAGE_NOT_32_BYTES: &[u8] = "32 bytes of data expected in storage at key".as_bytes();
