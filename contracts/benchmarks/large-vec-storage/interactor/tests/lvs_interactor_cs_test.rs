use large_vec_storage_interactor::{Config, LargeVecStorageInteractor};
use serial_test::serial;

/// Small smoke benchmark: all calls succeed, and preloading does not increase decode gas
/// (net of the `noop` baseline, which differs because of contract size).
#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn large_vec_storage_bench_cs_test() {
    let mut interact = LargeVecStorageInteractor::new(Config::chain_simulator_config()).await;
    let results = interact.bench(&[1, 10, 50]).await;
    println!("{}", results.to_markdown());

    for size_index in 0..results.sizes.len() {
        let default_gas = results.get_net(0, size_index, "decodeDefault").unwrap();
        let preload_gas = results.get_net(1, size_index, "decodeDefault").unwrap();
        assert!(
            preload_gas <= default_gas,
            "preload decode more expensive at n = {}",
            results.sizes[size_index]
        );
    }
}
