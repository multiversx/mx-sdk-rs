use multiversx_sc::contract_base::StorageRawWrapper;
use multiversx_sc_scenario::api::SingleTxApi;

#[test]
fn single_tx_api_test() {
    let storage_raw = StorageRawWrapper::<SingleTxApi>::new();
    let storage_key = "test-num";

    // unitialized, we get the default
    let x: i32 = storage_raw.read(storage_key);
    assert_eq!(x, 0);

    // write, as if from a contract
    storage_raw.write(storage_key, &5i32);
    let x: i32 = storage_raw.read(storage_key);
    assert_eq!(x, 5);

    // check directly in storage
    SingleTxApi::with_global_default_account(|account| {
        let value = account.storage.get(storage_key.as_bytes()).unwrap();
        assert_eq!(value, &vec![5u8]);

        // change value directly in storage
        account
            .storage
            .insert(storage_key.as_bytes().to_vec(), vec![7u8]);
    });

    // read again
    let x: i32 = storage_raw.read(storage_key);
    assert_eq!(x, 7);

    // clear everything, globally
    SingleTxApi::clear_global();
    let x: i32 = storage_raw.read(storage_key);
    assert_eq!(x, 0);

    // checking directly in storage
    SingleTxApi::with_global_default_account(|account| {
        let value = account
            .storage
            .get(storage_key.as_bytes())
            .cloned()
            .unwrap_or_default();
        assert!(value.is_empty());
    });
}
