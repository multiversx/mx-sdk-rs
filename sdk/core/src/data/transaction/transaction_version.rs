use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Minimum transaction version required to use [`TransactionOptions`].
pub const MIN_VERSION_FOR_OPTIONS: u32 = 2;

/// Transaction protocol version.
///
/// `V2` is the default and required for hash-signing and guardian features.
/// `V1` is kept for compatibility when deserialising legacy transactions.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionVersion {
    V1,
    #[default]
    V2,
}

impl TransactionVersion {
    /// Returns the numeric value (`1` or `2`).
    pub fn as_u32(self) -> u32 {
        match self {
            TransactionVersion::V1 => 1,
            TransactionVersion::V2 => 2,
        }
    }

    /// Returns `true` if this version supports [`TransactionOptions`].
    pub fn supports_options(self) -> bool {
        self.as_u32() >= MIN_VERSION_FOR_OPTIONS
    }
}

impl TryFrom<u32> for TransactionVersion {
    type Error = anyhow::Error;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(TransactionVersion::V1),
            2 => Ok(TransactionVersion::V2),
            other => Err(anyhow!("unknown transaction version: {other}")),
        }
    }
}

impl From<TransactionVersion> for u32 {
    fn from(v: TransactionVersion) -> Self {
        v.as_u32()
    }
}

impl Serialize for TransactionVersion {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(self.as_u32())
    }
}

impl<'de> Deserialize<'de> for TransactionVersion {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = u32::deserialize(d)?;
        TransactionVersion::try_from(v).map_err(serde::de::Error::custom)
    }
}
