pub use env_logger;
pub use hex;
pub use log;
pub use multiversx_sc_scenario::{self, multiversx_sc};
pub use multiversx_sdk_dapp as sdk;

/// Imports normally needed in interactors, grouped together.
pub mod imports;

pub type DappInteractor =
    multiversx_sc_snippets_base::InteractorBase<multiversx_sdk_dapp::GatewayDappProxy>;
