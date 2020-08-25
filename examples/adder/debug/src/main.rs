use adder::*;
use elrond_wasm_debug::*;
use elrond_wasm_debug::HashMap;

static ADDR1: [u8; 32] = [0x11u8; 32];
static ADDR2: [u8; 32] = [0x22u8; 32];

fn main() {
    let mock_ref = ArwenMockState::new_ref();

    mock_ref.add_account(AccountData{
        address: ADDR1.into(),
        nonce: 0,
        balance: 0.into(),
        storage: HashMap::new(),
        contract: None,
    });

    // tx 1: init
    let mut tx1 = TxData::new_create(
        Box::new(AdderImpl::new(mock_ref.clone())), 
        ADDR1.into(), 
        ADDR2.into());
    tx1.add_arg(vec![7u8]);
    let result1 = mock_ref.execute_tx(tx1);
    assert_eq!(0, result1.result_status);
    result1.print();

    // tx 2: add!
    let mut tx2 = TxData::new_call(
        "add", 
        ADDR1.into(), 
        ADDR2.into());
    tx2.add_arg(vec![2u8]);
    let result2 = mock_ref.execute_tx(tx2);
    assert_eq!(0, result2.result_status);
    //result2.print();

    // tx 3: getSum
    let tx3 = TxData::new_call(
        "getSum", 
        ADDR1.into(), 
        ADDR2.into());
    let result3 = mock_ref.execute_tx(tx3);
    result3.print();

    mock_ref.print_accounts();
}
