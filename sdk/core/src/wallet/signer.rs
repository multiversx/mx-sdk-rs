use super::PrivateKey;

/// The mechanism used to sign transactions and messages.
///
/// Always compiled — no `#[cfg]` on the type or its variants.
/// The `Ledger` variant holds only a `u32` address index, so there is no
/// compile-time dependency on `libhidapi`.
///
/// Attempting to *sign* with `Signer::Ledger` when the `ledger` feature is
/// not active will return an informative runtime error from [`Wallet::sign_tx`]
/// and [`Wallet::sign_bytes`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Signer {
    /// Signs locally using an in-memory ed25519 private key.
    PrivateKey(Box<PrivateKey>),

    /// Signs on a connected Ledger hardware device at the given BIP-44 address
    /// index (account is always 0 for MultiversX).
    Ledger { address_index: u32 },
}
