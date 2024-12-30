use multiversx_sc::api::ESDT_MULTI_TRANSFER_FUNC_NAME;
use multiversx_sc_scenario::scenario_model::ScCallStep;
use num_traits::Zero;

#[test]
fn test_tx_call_normalize_single_esdt_token_fungible() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:WEGLD-abcdef", 0, 10u32)
        .function("func");

    assert_eq!(
        tx.tx.normalize().compute_data_field(),
        "ESDTTransfer@5745474c442d616263646566@0a@66756e63",
    );
}

#[test]
fn test_tx_call_normalize_single_esdt_token_non_fungible() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:SFT-abcdef", 1, 10u32)
        .function("func");

    assert_eq!(
        tx.tx.normalize().compute_data_field(),
        "ESDTNFTTransfer@5346542d616263646566@01@0a@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@66756e63",
    );
}

/// Only MultiESDTNFTTransfer supports EGLD-000000, so it is used even though we have a single token transfer.
#[test]
fn test_tx_call_normalize_single_egld_000000() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:EGLD-000000", 0, 10u32)
        .function("func");

    assert_eq!(
        tx.tx.normalize().compute_data_field(),
        "MultiESDTNFTTransfer@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@01@45474c442d303030303030@@0a@66756e63",
    );
}

#[test]
fn test_tx_call_normalize_multi_esdt_1() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:WEGLD-abcdef", 0, 10u32)
        .esdt_transfer("str:USDC-abcdef", 0, 11u32);

    assert_eq!(
        tx.tx.normalize().compute_data_field(),
        "MultiESDTNFTTransfer@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@02@5745474c442d616263646566@@0a@555344432d616263646566@@0b",
    );
}

#[test]
fn test_tx_call_normalize_multi_esdt_2() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:WEGLD-abcdef", 0, 10u32)
        .esdt_transfer("str:USDC-abcdef", 0, 11u32)
        .function("func");

    assert_eq!(
        tx.tx.normalize().compute_data_field(),
        "MultiESDTNFTTransfer@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@02@5745474c442d616263646566@@0a@555344432d616263646566@@0b@66756e63",
    );
}

#[test]
fn test_tx_call_normalize_single_esdt_token() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:WEGLD-abcdef", 0, 10u32)
        .function("func");

    assert_eq!(
        tx.tx.normalize().compute_data_field(),
        "ESDTTransfer@5745474c442d616263646566@0a@66756e63",
    );
}

#[test]
#[allow(deprecated)]
fn test_contract_call_multi_esdt_deprecated() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .esdt_transfer("str:WEGLD-abcdef", 0, 10u32)
        .esdt_transfer("str:USDC-abcdef", 0, 11u32);

    let cc = tx.tx.to_contract_call();

    assert_eq!(
        cc.basic.function_call.function_name.to_vec(),
        ESDT_MULTI_TRANSFER_FUNC_NAME.as_bytes().to_vec(),
    );
    assert_eq!(
        cc.to_call_data_string().to_string(),
        "MultiESDTNFTTransfer@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@02@5745474c442d616263646566@@0a@555344432d616263646566@@0b",
    );
    assert!(tx.tx.egld_value.value.is_zero());
    assert_eq!(tx.tx.from.value, cc.basic.to.to_address());
}
