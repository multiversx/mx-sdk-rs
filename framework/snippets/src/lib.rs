pub use env_logger;
pub use hex;
pub use log;
pub use multiversx_sc_scenario::{self, multiversx_sc};
pub use multiversx_sc_snippets_base as base;
pub use multiversx_sdk as sdk_core;
pub use multiversx_sdk as sdk;
pub use tokio;

/// Imports normally needed in interactors, grouped together.
pub mod imports;

/// Backwards compatibility.
pub use crate::sdk_core::test_wallets;

pub type HttpInteractor =
    multiversx_sc_snippets_base::InteractorBase<multiversx_sdk_http::GatewayHttpProxy>;

/// Backwards compatibility.
pub type Interactor = HttpInteractor;
