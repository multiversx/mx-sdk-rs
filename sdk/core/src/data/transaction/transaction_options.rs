use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags::bitflags! {
    /// Bit-flags that modify how a transaction is signed and processed.
    ///
    /// | Bit | Constant         | Value | Meaning                                           |
    /// |-----|------------------|-------|---------------------------------------------------|
    /// |  0  | `SIGN_WITH_HASH` | `1`   | Sign over keccak256 hash instead of raw JSON.     |
    /// |  1  | `GUARDED`        | `2`   | Transaction has a guardian co-signer.             |
    ///
    /// Requires [`TransactionVersion::V2`] or higher.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct TransactionOptions: u32 {
        /// No special options (default for version-1 transactions).
        const DEFAULT        = 0b0000;

        /// Sign over the keccak256 hash of the serialised transaction.
        /// Required when signing with a Ledger hardware wallet.
        const SIGN_WITH_HASH = 0b0001;

        /// Transaction is protected by a guardian (co-signer).
        const GUARDED        = 0b0010;
    }
}

impl Serialize for TransactionOptions {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for TransactionOptions {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bits = u32::deserialize(d)?;
        Ok(TransactionOptions::from_bits_retain(bits))
    }
}

impl TransactionOptions {
    pub fn sign_with_hash(&self) -> bool {
        self.contains(TransactionOptions::SIGN_WITH_HASH)
    }
}
