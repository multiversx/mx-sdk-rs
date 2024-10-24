use multiversx_sdk::gateway::NetworkConfigRequest;
use multiversx_sdk_http::{GatewayHttpProxy, CHAIN_SIMULATOR_GATEWAY};

#[tokio::test]
#[cfg_attr(not(feature = "chain_simulator"), ignore)]
async fn get_network_config_test() {
    let blockchain = GatewayHttpProxy::new(CHAIN_SIMULATOR_GATEWAY.to_string());
    let network_config = blockchain.http_request(NetworkConfigRequest).await.unwrap();

    assert_eq!(network_config.chain_id, "chain");
    assert_eq!(network_config.denomination, 18);
    assert_eq!(network_config.num_shards_without_meta, 3);
    assert_eq!(network_config.round_duration, 6000);
    assert_eq!(network_config.shard_consensus_group_size, 1);
}
