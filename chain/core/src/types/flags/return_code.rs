const MESSAGE_OK: &str = "ok";
const MESSAGE_FUNCTION_NOT_FOUND: &str = "function not found";
const MESSAGE_WRONG_SIGNATURE: &str = "wrong signature for function";
const MESSAGE_CONTRACT_NOT_FOUND: &str = "contract not found";
const MESSAGE_USER_ERROR: &str = "user error";
const MESSAGE_OUT_OF_GAS: &str = "out of gas";
const MESSAGE_ACCOUNT_COLLISION: &str = "account collision";
const MESSAGE_OUT_OF_FUNDS: &str = "out of funds";
const MESSAGE_CALL_STACK_OVERFLOW: &str = "call stack overflow";
const MESSAGE_CONTRACT_INVALID: &str = "contract invalid";
const MESSAGE_EXECUTION_FAILED: &str = "execution failed";
const MESSAGE_UNKNOWN_ERROR: &str = "unknown error";
const MESSAGE_NETWORK_TIMEOUT: &str = "network timeout";

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum ReturnCode {
    /// Returned when execution was completed normally.
    #[default]
    Success = 0,

    /// Returned when the input specifies a function name that does not exist or is not public.
    FunctionNotFound = 1,

    /// Returned when the wrong number of arguments is provided.
    FunctionWrongSignature = 2,

    /// Returned when the called contract does not exist.
    ContractNotFound = 3,

    /// Returned for various execution errors.
    UserError = 4,

    /// Returned when VM execution runs out of gas.
    OutOfGas = 5,

    /// Returned when created account already exists.
    AccountCollision = 6,

    /// Returned when the caller (sender) runs out of funds.
    OutOfFunds = 7,

    /// Returned when stack overflow occurs.
    CallStackOverFlow = 8,

    /// Returned when the contract is invalid.
    ContractInvalid = 9,

    /// Returned when the execution of the specified function has failed.
    ExecutionFailed = 10,

    /// Returned when the upgrade of the contract has failed
    UpgradeFailed = 11,

    /// Returned when tx simulation fails execution
    SimulateFailed = 12,

    /// Only occurs in the debugger context.
    DebugApiError = 100,

    // Returned when a connection timeout occurs
    NetworkTimeout = 101,
}

impl ReturnCode {
    pub fn as_u64(self) -> u64 {
        self as u64
    }

    pub fn is_success(self) -> bool {
        self == ReturnCode::Success
    }

    pub fn message(self) -> &'static str {
        match self {
            ReturnCode::Success => MESSAGE_OK,
            ReturnCode::FunctionNotFound => MESSAGE_FUNCTION_NOT_FOUND,
            ReturnCode::FunctionWrongSignature => MESSAGE_WRONG_SIGNATURE,
            ReturnCode::ContractNotFound => MESSAGE_CONTRACT_NOT_FOUND,
            ReturnCode::UserError => MESSAGE_USER_ERROR,
            ReturnCode::OutOfGas => MESSAGE_OUT_OF_GAS,
            ReturnCode::AccountCollision => MESSAGE_ACCOUNT_COLLISION,
            ReturnCode::OutOfFunds => MESSAGE_OUT_OF_FUNDS,
            ReturnCode::CallStackOverFlow => MESSAGE_CALL_STACK_OVERFLOW,
            ReturnCode::ContractInvalid => MESSAGE_CONTRACT_INVALID,
            ReturnCode::ExecutionFailed => MESSAGE_EXECUTION_FAILED,
            ReturnCode::NetworkTimeout => MESSAGE_NETWORK_TIMEOUT,
            _ => MESSAGE_UNKNOWN_ERROR,
        }
    }

    pub fn from_u64(value: u64) -> Option<ReturnCode> {
        match value {
            0 => Some(ReturnCode::Success),
            1 => Some(ReturnCode::FunctionNotFound),
            2 => Some(ReturnCode::FunctionWrongSignature),
            3 => Some(ReturnCode::ContractNotFound),
            4 => Some(ReturnCode::UserError),
            5 => Some(ReturnCode::OutOfGas),
            6 => Some(ReturnCode::AccountCollision),
            7 => Some(ReturnCode::OutOfFunds),
            8 => Some(ReturnCode::CallStackOverFlow),
            9 => Some(ReturnCode::ContractInvalid),
            10 => Some(ReturnCode::ExecutionFailed),
            11 => Some(ReturnCode::UpgradeFailed),
            12 => Some(ReturnCode::SimulateFailed),
            100 => Some(ReturnCode::DebugApiError),
            101 => Some(ReturnCode::NetworkTimeout),
            _ => None,
        }
    }

    pub fn from_message(message: &str) -> Option<ReturnCode> {
        match message {
            MESSAGE_OK => Some(ReturnCode::Success),
            MESSAGE_FUNCTION_NOT_FOUND => Some(ReturnCode::FunctionNotFound),
            MESSAGE_WRONG_SIGNATURE => Some(ReturnCode::FunctionWrongSignature),
            MESSAGE_CONTRACT_NOT_FOUND => Some(ReturnCode::ContractNotFound),
            MESSAGE_USER_ERROR => Some(ReturnCode::UserError),
            MESSAGE_OUT_OF_GAS => Some(ReturnCode::OutOfGas),
            MESSAGE_ACCOUNT_COLLISION => Some(ReturnCode::AccountCollision),
            MESSAGE_OUT_OF_FUNDS => Some(ReturnCode::OutOfFunds),
            MESSAGE_CALL_STACK_OVERFLOW => Some(ReturnCode::CallStackOverFlow),
            MESSAGE_CONTRACT_INVALID => Some(ReturnCode::ContractInvalid),
            MESSAGE_EXECUTION_FAILED => Some(ReturnCode::ExecutionFailed),
            MESSAGE_NETWORK_TIMEOUT => Some(ReturnCode::NetworkTimeout),
            _ => None,
        }
    }
}

impl core::fmt::Display for ReturnCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_u64().fmt(f)
    }
}
