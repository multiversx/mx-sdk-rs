use super::TxFunctionName;

/// Structure used to accumulate errors through function calls.
#[derive(Clone, Debug)]
pub struct TxErrorTrace {
    pub function_name: TxFunctionName,
    pub error_trace_message: String,
    pub additional_info: Vec<String>,
}
