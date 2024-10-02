mod gateway_account;
mod gateway_block;
mod gateway_chain_simulator;
mod gateway_network;
mod gateway_proxy;
mod gateway_tx;
mod gateway_tx_retrieve;

pub use gateway_proxy::GatewayHttpProxy;
use multiversx_sdk::gateway::GatewayAsyncService;

pub const MAINNET_GATEWAY: &str = "https://gateway.multiversx.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.multiversx.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.multiversx.com";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

pub const DEFAULT_USE_CHAIN_SIMULATOR: bool = false;

impl GatewayAsyncService for GatewayHttpProxy {
    fn request<G>(
        &self,
        request: G,
    ) -> impl std::future::Future<Output = anyhow::Result<G::Result>> + Send
    where
        G: multiversx_sdk::gateway::GatewayRequest,
    {
        self.http_request(request)
    }
}
