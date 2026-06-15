use crate::sdk::wallet::Wallet;

use super::ConnectionConfig;

/// Trait implemented by a contract-specific `Config` struct to tell the
/// [`HttpInteractorBuilder`](crate::HttpInteractorBuilder) how to initialise the interactor.
///
/// The builder calls `connection()` to open the gateway and calls `register_wallets()` to
/// register every wallet that will sign transactions.
pub trait InteractorConfig {
    /// Returns the connection settings (gateway URI, chain type).
    fn connection(&self) -> &ConnectionConfig;

    /// Returns the wallets to be registered with the interactor during setup.
    ///
    /// Optional — defaults to no wallets. Additional wallets can always be
    /// registered manually on the interactor after construction.
    fn register_wallets(&self) -> Vec<Wallet> {
        Vec::new()
    }
}

/// A bare [`ConnectionConfig`] can be used directly as an [`InteractorConfig`]
/// when no wallets or custom directory are needed.
impl InteractorConfig for ConnectionConfig {
    fn connection(&self) -> &ConnectionConfig {
        self
    }
}
