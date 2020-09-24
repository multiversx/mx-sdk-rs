use simple_erc20::*;
use elrond_wasm_debug::*;
use elrond_wasm_debug::HashMap;

fn main() {
    let mock_ref = ArwenMockState::new_ref();

    mock_ref.add_account(AccountData{
        address: [0x11u8; 32].into(),
        nonce: 0,
        balance: 0u32.into(),
        storage: HashMap::new(),
        contract: None,
    });

    // tx 1: init
    let mut tx1 = TxData::new_create(
        Box::new(SimpleErc20TokenImpl::new(mock_ref.clone())), 
        [0x11u8; 32].into(), 
        [0x22u8; 32].into());
    tx1.add_arg(vec![100u8]);
    let result1 = mock_ref.execute_tx(tx1);
    assert_eq!(0, result1.result_status);
    result1.print();

    // tx 2: transfer
    let mut tx2 = TxData::new_call(
        "transferToken", 
        [0x11u8; 32].into(), 
        [0x22u8; 32].into());
    tx2.add_arg(vec![0x33u8; 32]); // to
    tx2.add_arg(vec![1u8]); // amount
    let result2 = mock_ref.execute_tx(tx2);
    assert_eq!(0, result2.result_status);
    //result2.print();

    // tx 3: balance
    let mut tx3 = TxData::new_call(
        "balanceOf", 
        [0x11u8; 32].into(), 
        [0x22u8; 32].into());
    tx3.add_arg(vec![0x33u8; 32]); // subject
    let result3 = mock_ref.execute_tx(tx3);
    result3.print();

    mock_ref.print_accounts();
}
