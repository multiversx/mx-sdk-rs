# Elrond SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/mx-sdk-erdrs)](https://crates.io/crates/mx-sdk-erdrs)

## Example

```rust
use mx_sdk_erdrs::blockchain::rpc::{ElrondProxy, DEVNET_GATEWAY};

#[tokio::test]
async fn get_network_config() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}
```

More example in `./src/blockchain/tests.rs`