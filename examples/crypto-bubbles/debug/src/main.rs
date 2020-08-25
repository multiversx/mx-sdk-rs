#![allow(unused_attributes)]

use crypto_bubbles::*;
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
    let tx1 = TxData::new_create(
        Box::new(CryptoBubblesImpl::new(mock_ref.clone())), 
        ADDR1.into(), 
        ADDR2.into());
    mock_ref.execute_tx(tx1);

}
