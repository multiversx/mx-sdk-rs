mod gateway_http_proxy;

pub use gateway_http_proxy::GatewayHttpProxy;

pub use multiversx_sdk as core;

pub const MAINNET_GATEWAY: &str = "https://gateway.multiversx.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.multiversx.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.multiversx.com";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

pub const DEFAULT_USE_CHAIN_SIMULATOR: bool = false;
