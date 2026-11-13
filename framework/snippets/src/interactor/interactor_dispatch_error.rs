use std::fmt;

use multiversx_sc_scenario::scenario_model::TxResponseStatus;
use multiversx_sdk::chain_core::types::GasResult;

use super::TxOutputTarget;

/// Everything that can prevent `dispatch()` from producing the handled result of a transaction.
///
/// On-chain failures only end up here when no result handler took responsibility for them
/// (e.g. `ReturnsHandledOrError` or `ExpectError`).
#[derive(Debug)]
pub enum DispatchError {
    /// The transaction was written and/or sent, but not executed, or its result was not awaited.
    ///
    /// Not a failure: the requested work was done, but no result can be produced.
    Interrupted(TxInterrupted),

    /// The transaction executed and failed, while the default expectation (success) was in place.
    TxFailed(TxResponseStatus),

    /// A result handler rejected the response.
    ResultHandler(ResultHandlerError),

    /// Gas simulation could not produce an estimate (failed or out of gas).
    Simulate(GasResult),

    /// Broadcasting, fetching the transaction, generating blocks, or any other gateway failure.
    Gateway(anyhow::Error),

    /// Writing the transaction output.
    Io(std::io::Error),
}

impl DispatchError {
    /// True if the transaction was written and/or sent as requested, but not executed.
    pub fn is_interrupted(&self) -> bool {
        matches!(self, DispatchError::Interrupted(_))
    }
}

impl fmt::Display for DispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DispatchError::Interrupted(interrupted) => interrupted.fmt(f),
            DispatchError::TxFailed(status) => status.fmt(f),
            DispatchError::ResultHandler(err) => err.fmt(f),
            DispatchError::Simulate(gas_result) => match gas_result {
                GasResult::GasUsed(gas) => write!(f, "unexpected gas simulation result: {gas}"),
                GasResult::OutOfGas => write!(f, "gas simulation failed: out of gas"),
                GasResult::Failed(message) => write!(f, "gas simulation failed: {message}"),
            },
            DispatchError::Gateway(err) => write!(f, "gateway error: {err:#}"),
            DispatchError::Io(err) => write!(f, "failed to write transaction output: {err}"),
        }
    }
}

impl std::error::Error for DispatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DispatchError::Gateway(err) => Some(err.as_ref()),
            DispatchError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TxInterrupted> for DispatchError {
    fn from(interrupted: TxInterrupted) -> Self {
        DispatchError::Interrupted(interrupted)
    }
}

impl From<ResultHandlerError> for DispatchError {
    fn from(err: ResultHandlerError) -> Self {
        DispatchError::ResultHandler(err)
    }
}

impl From<std::io::Error> for DispatchError {
    fn from(err: std::io::Error) -> Self {
        DispatchError::Io(err)
    }
}

/// What happened to a transaction that was not executed, or whose result was not awaited.
///
/// Writing and sending are orthogonal: a transaction can be written, sent, or both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxInterrupted {
    /// Where the signed transaction was written (a copy of the setting).
    pub output: TxOutputTarget,

    /// The hash, if the transaction was broadcast (its result was not awaited).
    pub tx_hash: Option<String>,
}

impl fmt::Display for TxInterrupted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transaction not executed")?;
        match &self.output {
            TxOutputTarget::NotWritten => {}
            TxOutputTarget::Stdout => write!(f, ", written to stdout")?,
            TxOutputTarget::File(path) => write!(f, ", written to {}", path.display())?,
        }
        match &self.tx_hash {
            Some(tx_hash) => write!(f, ", sent with hash {tx_hash}, result not awaited"),
            None => write!(f, ", not sent"),
        }
    }
}

/// A result handler rejected the transaction response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultHandlerError {
    /// An explicit expectation (`ExpectError`, `ExpectStatus`, `ExpectMessage`, ...) was not met.
    ExpectationMismatch {
        /// What was checked: "status", "message", "out", "logs", ...
        field: &'static str,
        expected: String,
        actual: String,
    },
}

impl fmt::Display for ResultHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultHandlerError::ExpectationMismatch {
                field,
                expected,
                actual,
            } => write!(
                f,
                "transaction {field} mismatch: expected {expected}, got {actual}"
            ),
        }
    }
}

impl std::error::Error for ResultHandlerError {}
