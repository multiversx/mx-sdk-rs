use basic_features_interact::{BasicFeaturesInteract, Config};
use multiversx_sc_snippets::imports::{
    BigUint, ConstDecimals, ManagedDecimal, ManagedOption, RustBigUint, StaticApi,
};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_basic_features_test() {
    let mut bf_interact = BasicFeaturesInteract::init(Config::chain_simulator_config()).await;

    bf_interact.deploy_storage_bytes().await;
    bf_interact.large_storage(15).await;

    let data = bf_interact.get_large_storage().await.to_vec();
    assert_eq!(bf_interact.large_storage_payload, data);

    bf_interact.deploy().await;

    let expected_return_egld_decimal =
        ManagedDecimal::<StaticApi, ConstDecimals<18>>::const_decimals_from_raw(BigUint::from(
            5u64,
        ));
    let return_egld_decimal = bf_interact.returns_egld_decimal(5).await;
    assert_eq!(expected_return_egld_decimal, return_egld_decimal);

    let expected_type_managed_option = ManagedOption::some(BigUint::from(8u16));
    let type_managed_option = bf_interact
        .echo_managed_option(expected_type_managed_option)
        .await;
    assert_eq!(Some(RustBigUint::from(8u16)), type_managed_option);
}
