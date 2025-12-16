// TODO: remove once minimum version is 1.87+
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_is_multiple_of)]

pub mod account_tool;
mod interactor;
mod multi;
pub mod network_response;

pub use env_logger;
pub use hex;
pub use interactor::*;
pub use log;
pub use multi::*;
pub use multiversx_sc_scenario::{self, multiversx_sc};
pub use multiversx_sdk as sdk_core;
pub use multiversx_sdk as sdk;

/// Imports normally needed in interactors, grouped together.
pub mod imports;

/// Backwards compatibility.
pub use crate::sdk_core::test_wallets;

#[cfg(feature = "http")]
pub type HttpInteractor = crate::InteractorBase<multiversx_sdk_http::GatewayHttpProxy>;

/// Backwards compatibility.
#[cfg(feature = "http")]
pub type Interactor = HttpInteractor;

#[cfg(feature = "dapp")]
pub type DappInteractor = crate::InteractorBase<multiversx_sdk_dapp::GatewayDappProxy>;
