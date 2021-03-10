mod async_call_result;
mod multi_args;
mod multi_result;
mod multi_result_vec;
mod optional_arg;
mod optional_result;
mod sc_error;
mod sc_result;
mod var_args;

pub use async_call_result::{AsyncCallError, AsyncCallResult};
pub use multi_args::*;
pub use multi_result::*;
pub use multi_result_vec::MultiResultVec;
pub use optional_arg::OptionalArg;
pub use optional_result::OptionalResult;
pub use sc_error::SCError;
pub use sc_result::SCResult;
pub use var_args::VarArgs;
