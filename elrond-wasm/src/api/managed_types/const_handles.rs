use super::Handle;

/// Used as a flag. Reading from this handle will always result in a crash.
///
/// Do not initialize!
pub const UNINITIALIZED_HANDLE: Handle = i32::MAX;

/// WARNING! With the current VM this still needs to be initialized before use.
pub const BIG_INT_CONST_ZERO: Handle = -10;

pub const CALL_VALUE_EGLD: Handle = -11;
pub const CALL_VALUE_SINGLE_ESDT: Handle = -13;

pub const BIG_INT_TEMPORARY_1: Handle = -14;
pub const BIG_INT_TEMPORARY_2: Handle = -15;

/// WARNING! With the current VM this still needs to be initialized before use.
pub const MBUF_CONST_EMPTY: Handle = -20;
pub const CALL_VALUE_MULTI_ESDT: Handle = -21;
pub const CALL_VALUE_SINGLE_ESDT_TOKEN_NAME: Handle = -22;
pub const MBUF_TEMPORARY_1: Handle = -25;
pub const MBUF_TEMPORARY_2: Handle = -26;

pub const NEW_HANDLE_START_FROM: Handle = -100; // > -100 reserved for APIs

/// Used as a flag. Do not use as a regular handle.
pub const MANAGED_OPTION_NONE: Handle = i32::MAX - 1;
