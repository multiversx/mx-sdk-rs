pub const NON_PAYABLE: &[u8] = b"attempted to transfer funds via a non-payable function";

pub const ARG_WRONG_NUMBER: &[u8] = b"wrong number of arguments";
pub const ARG_ASYNC_WRONG_NUMBER: &[u8] = b"wrong number of arguments provided to async call";
pub const ARG_ASYNC_RETURN_WRONG_NUMBER: &[u8] =
	b"wrong number of arguments returned by async call";
pub const ARG_CALLBACK_TOO_FEW: &[u8] = b"too few callback arguments provided";
pub const ARG_CALLBACK_TOO_MANY: &[u8] = b"too many callback arguments provided";

pub const ARG_OUT_OF_RANGE: &[u8] = b"argument out of range";
pub const ARG_BAD_LENGTH: &[u8] = b"argument has wrong length";
pub const ARG_BAD_LENGTH_32: &[u8] = b"argument has wrong length: 32 bytes expected";
pub const ARG_DECODE_ERROR_1: &[u8] = b"argument decode error (";
pub const ARG_DECODE_ERROR_2: &[u8] = b"): ";
pub const STORAGE_VALUE_OUT_OF_RANGE: &[u8] = b"storage value out of range";
pub const STORAGE_DECODE_ERROR: &[u8] = b"storage decode error: ";

pub const BIG_UINT_EXCEEDS_SLICE: &[u8] = b"big uint as_bytes exceed target slice";
pub const BIG_UINT_SUB_NEGATIVE: &[u8] = b"cannot subtract because result would be negative";

pub const DESERIALIZATION_INVALID_BYTE: &[u8] =
	b"call data deserialization error: not a valid byte";
pub const DESERIALIZATION_NOT_32_BYTES: &[u8] =
	b"call data deserialization error: 32 as_bytes expected";
pub const DESERIALIZATION_ODD_DIGITS: &[u8] =
	b"call data deserialization error: odd number of digits in hex representation";
pub const DESERIALIZATION_ARG_OUT_OF_RANGE: &[u8] =
	b"call data deserialization error: argument out of range";

pub const CALLBACK_BAD_FUNC: &[u8] = b"no callback function with that name exists in contract";

pub const STORAGE_NOT_I64: &[u8] = b"storage not i64";
pub const STORAGE_NOT_32_BYTES: &[u8] = b"32 bytes of data expected in storage at key";
