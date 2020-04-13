
pub static NON_PAYABLE: &str = "attempted to transfer funds via a non-payable function";

pub static ARG_WRONG_NUMBER: &str = "wrong number of arguments";
pub static ARG_ASYNC_RETURN_WRONG_NUMBER: &str = "wrong number of arguments returned by async call";
pub static ARG_CALLBACK_TOO_FEW:   &str = "too few callback arguments provided";
pub static ARG_CALLBACK_TOO_MANY:  &str = "too many callback arguments provided";

pub static ARG_OUT_OF_RANGE: &str = "argument out of range";
pub static ARG_BAD_LENGTH: &str = "argument has wrong length";
pub static ARG_BAD_LENGTH_32: &str = "argument has wrong length: 32 bytes expected";

pub static BIG_UINT_EXCEEDS_SLICE: &str = "big uint bytes exceed target slice";
pub static BIG_UINT_SUB_NEGATIVE: &str = "cannot subtract because result would be negative";

pub static DESERIALIZATION_INVALID_BYTE: &str = "call data deserialization error: not a valid byte";
pub static DESERIALIZATION_NOT_32_BYTES: &str = "call data deserialization error: 32 bytes expected";
pub static DESERIALIZATION_ODD_DIGITS: &str = "call data deserialization error: odd number of digits in hex representation";
pub static DESERIALIZATION_ARG_OUT_OF_RANGE: &str = "call data deserialization error: argument out of range";
