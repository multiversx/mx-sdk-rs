use simple_erc20::*;
use elrond_wasm_debug::*;
use elrond_wasm_debug::HashMap;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/simple-erc20.wasm",
        Box::new(|mock_ref| Box::new(BasicFeaturesImpl::new(mock_ref))));
    contract_map
}

fn main() {
    let contract_map = contract_map();
    
    let mock_ref = ArwenMockState::new_ref();

    mock_ref.add_account(AccountData{
        address: [0x11u8; 32].into(),
        nonce: 0,
        balance: 0u32.into(),
        storage: HashMap::new(),
        contract_path: None,
        contract_owner: None,
    });

    // tx 1: init
    let mut tx1 = TxData::new_create(
        b"file:../output/simple-erc20.wasm".to_vec(),
        [0x11u8; 32].into(), 
        [0x22u8; 32].into());
    tx1.add_arg(vec![100u8]);
    let result1 = mock_ref.execute_tx(tx1, &contract_map);
    assert_eq!(0, result1.result_status);
    result1.print();

    // tx 2: transfer
    let mut tx2 = TxData::new_call(
        "transferToken", 
        [0x11u8; 32].into(), 
        [0x22u8; 32].into());
    tx2.add_arg(vec![0x33u8; 32]); // to
    tx2.add_arg(vec![1u8]); // amount
    let result2 = mock_ref.execute_tx(tx2, &contract_map);
    assert_eq!(0, result2.result_status);
    //result2.print();

    // tx 3: balance
    let mut tx3 = TxData::new_call(
        "balanceOf", 
        [0x11u8; 32].into(), 
        [0x22u8; 32].into());
    tx3.add_arg(vec![0x33u8; 32]); // subject
    let result3 = mock_ref.execute_tx(tx3, &contract_map);
    result3.print();

    mock_ref.print_accounts();
}
