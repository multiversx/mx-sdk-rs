use std::fmt;

/// Errors that can be returned by [`LedgerApp`](super::LedgerApp) operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    /// The device could not be found or opened (not plugged in, locked, or wrong app).
    DeviceNotFound,
    /// The device returned a non-success status word.
    StatusWord(u16),
    /// A lower-level HID transport error.
    Transport(String),
    /// The APDU response had an unexpected format.
    InvalidResponse(String),
}

impl LedgerError {
    pub(super) fn from_status_word(sw: u16) -> Self {
        LedgerError::StatusWord(sw)
    }
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerError::DeviceNotFound => write!(
                f,
                "Ledger device not found; check that it is plugged in, unlocked, and on the MultiversX app"
            ),
            LedgerError::StatusWord(sw) => {
                write!(f, "Ledger error (0x{sw:04X}): {}", status_word_message(*sw))
            }
            LedgerError::Transport(msg) => write!(f, "Ledger transport error: {msg}"),
            LedgerError::InvalidResponse(msg) => write!(f, "Ledger invalid response: {msg}"),
        }
    }
}

impl std::error::Error for LedgerError {}

fn status_word_message(sw: u16) -> &'static str {
    match sw {
        0x9000 => "success",
        0x6985 => "user denied",
        0x6D00 => "unknown instruction",
        0x6E00 => "wrong CLA",
        0x6E10 => "signature failed",
        0x6E01 => "invalid arguments",
        0x6E02 => "invalid message",
        0x6E03 => "invalid p1",
        0x6E04 => "message too long",
        0x6E05 => "receiver too long",
        0x6E06 => "amount too long",
        0x6E07 => "contract data disabled",
        0x6E08 => "message incomplete",
        0x6E09 => "wrong tx version",
        0x6E0A => "nonce too long",
        0x6E0B => "invalid amount",
        0x6E0C => "invalid fee",
        0x6E0D => "pretty failed",
        0x6E0E => "data too long",
        0x6E0F => "wrong tx options",
        0x6E11 => "regular signing is deprecated",
        _ => "unknown error",
    }
}
