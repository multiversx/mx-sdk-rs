mod gateway_dapp_proxy;
pub use gateway_dapp_proxy::GatewayDappProxy;

pub use multiversx_sdk as core;

pub use multiversx_sdk::{data, wallet};

// Not used directly: the dependency exists solely to enable getrandom's
// `wasm_js` feature, which selects the Crypto.getRandomValues backend for
// wasm32-unknown-unknown. Keeps `cargo::unused_dependencies` quiet.
use getrandom as _;
