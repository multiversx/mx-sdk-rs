use std::path::PathBuf;

/// Settings for what happens to a transaction after it is built and signed:
/// where it is written, whether it is broadcast, and whether to wait for the result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxDispatchSettings {
    /// Where to write the signed transaction (and, if awaited, `transactionOnNetwork`).
    pub output: TxOutputTarget,

    /// `None`: the transaction is not broadcast.
    pub send: Option<TxSendSettings>,
}

/// Default: send and wait for the result, output not written.
impl Default for TxDispatchSettings {
    fn default() -> Self {
        TxDispatchSettings {
            output: TxOutputTarget::NotWritten,
            send: Some(TxSendSettings { wait_result: true }),
        }
    }
}

/// Settings that only make sense when the transaction is broadcast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxSendSettings {
    /// Wait for the transaction to be processed and retrieve its result.
    pub wait_result: bool,
}

/// Where the signed transaction (the [`TxOutputFile`](crate::TxOutputFile) JSON) is written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TxOutputTarget {
    /// Not written anywhere.
    #[default]
    NotWritten,
    Stdout,
    File(PathBuf),
}

impl TxOutputTarget {
    /// The given file, or stdout if none is given.
    pub fn file_or_stdout(outfile: Option<PathBuf>) -> Self {
        match outfile {
            Some(path) => TxOutputTarget::File(path),
            None => TxOutputTarget::Stdout,
        }
    }
}
