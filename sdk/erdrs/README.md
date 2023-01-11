# MultiversX SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/mx-sdk-erdrs)](https://crates.io/crates/mx-sdk-erdrs)

## Example

```rust
use mx_sdk_erdrs::blockchain::rpc::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::test]
async fn get_network_config() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}
```

More examples in `./examples`.