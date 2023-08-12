pub const PANIC_OCCURRED: &str = "panic occurred";
pub const MEM_ALLOC_ERROR: &str = "memory allocation error";

pub const NON_PAYABLE_FUNC_ESDT: &str = "function does not accept ESDT payment";
pub const BAD_TOKEN_PROVIDED: &str = "bad call value token provided";
pub const BAD_TOKEN_TICKER_FORMAT: &[u8] = b"bad token ticker format";
pub const SINGLE_ESDT_EXPECTED: &str = "function expects single ESDT payment";
pub const TOO_MANY_ESDT_TRANSFERS: &str = "too many ESDT transfers";
pub const ESDT_INVALID_TOKEN_INDEX: &str = "invalid token index";
pub const INCORRECT_NUM_ESDT_TRANSFERS: &str = "incorrect number of ESDT transfers";
pub static FUNGIBLE_TOKEN_EXPECTED_ERR_MSG: &str = "fungible ESDT token expected";

pub const ARG_WRONG_NUMBER: &str = "wrong number of arguments";
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
pub const STORAGE_ENCODE_ERROR: &[u8] = b"storage encode error: ";
pub const STORAGE_KEY_ENCODE_ERROR: &[u8] = b"storage key encode error: ";
pub const STORAGE_VALUE_EXCEEDS_BUFFER: &[u8] = b"storage value exceeds buffer";
pub const FINISH_ENCODE_ERROR: &[u8] = b"endpoint result encode error: ";
pub const SERIALIZER_DECODE_ERROR: &[u8] = b"serializer decode error: ";
pub const SERIALIZER_ENCODE_ERROR: &[u8] = b"serializer encode error: ";
pub const FORMATTER_ENCODE_ERROR: &[u8] = b"formatter encode error: ";
pub const LOG_TOPIC_ENCODE_ERROR: &[u8] = b"log topic encode error: ";
pub const LOG_DATA_ENCODE_ERROR: &[u8] = b"log data encode error: ";
pub const CONTRACT_CALL_ENCODE_ERROR: &[u8] = b"contract call encode error: ";

pub const VALUE_EXCEEDS_SLICE: &[u8] = b"value exceeds target slice";
pub const CAST_TO_I64_ERROR: &[u8] = b"cast to i64 error";
pub const BIG_UINT_EXCEEDS_SLICE: &[u8] = b"big uint as_bytes exceed target slice";
pub const BIG_UINT_SUB_NEGATIVE: &[u8] = b"cannot subtract because result would be negative";

pub const DESERIALIZATION_INVALID_BYTE: &str = "call data deserialization error: not a valid byte";
pub const DESERIALIZATION_NOT_32_BYTES: &str =
    "call data deserialization error: 32 as_bytes expected";
pub const DESERIALIZATION_ODD_DIGITS: &str =
    "call data deserialization error: odd number of digits in hex representation";
pub const DESERIALIZATION_ARG_OUT_OF_RANGE: &str =
    "call data deserialization error: argument out of range";

pub const CALLBACK_BAD_FUNC: &[u8] = b"no callback function with that name exists in contract";

pub const RECIPIENT_ADDRESS_NOT_SET: &str = "recipient address not set";
pub static ONLY_OWNER_CALLER: &[u8] = b"Endpoint can only be called by owner";
pub static ONLY_USER_ACCOUNT_CALLER: &[u8] = b"Endpoint can only be called by user accounts";

pub const STORAGE_NOT_I64: &[u8] = b"storage not i64";
pub const STORAGE_NOT_32_BYTES: &[u8] = b"32 bytes of data expected in storage at key";

/// An additional non-VM status, meant just to signal an error in the debugger infrastructure of in the tests.
pub const DEBUG_API_ERR_STATUS: u64 = 100;
pub const DEBUG_API_ERR_HANDLE_STALE: &str =
    "VMHooksApi misuse: operation called with a handle that does not point to the top context of the call stack";
pub const DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH: &str =
    "VMHooksApi misuse: operation called with handles from 2 different contexts";
