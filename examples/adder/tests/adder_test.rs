
extern crate adder;
use adder::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

// fn set_up_module_to_test() -> AdderImpl<ArwenMockRef, RustBigInt, RustBigUint> {
    
// }

#[test]
fn test_add() {
    let mut contract_factories = ContractFactories::new();
    contract_factories.register_contract(
        "file:../output/adder.wasm",
        Box::new(AdderFactory::new()));

    let mut state = ArwenMockState::new();
    state.add_account(AccountData{
        address: [3u8; 32].into(),
        nonce: 0,
        balance: 0u32.into(),
        storage: HashMap::new(),
        contract: Some(b"file:../output/adder.wasm".to_vec()),
    });


    // state.set_dummy_tx(&Address::zero());

    // let adder = AdderImpl::new(state.clone());
    

    // adder.init(&RustBigInt::from(5));
    // assert_eq!(RustBigInt::from(5), *adder.get_mut_sum());

    let tx_data = TxData{
        from: Address::zero(),
        to: Address::zero(),
        call_value: 0u32.into(),
        func_name: b"add".to_vec(),
        new_contract: None,
        args: vec![vec![5u8]],
    };
    state.current_tx = Some(tx_data);

    // let factory = {
    //     state.contract_factories.get(&"file:../output/adder.wasm".as_bytes().to_vec()).unwrap()
    // };
    // let adder = factory.new_contract(&mut state);
    let mut adder = contract_factories.new_contract("file:../output/adder.wasm", &mut state);
    adder.call(&b"add"[..]);
    // {
    //     let mut state = state.state_ref.borrow_mut();
    //     // state.create_account_if_deploy(&mut tx);    
    //     state.current_tx = Some(tx_data);
    //     state.clear_result();
    // }

    // {
    //     adder.call(&b"add"[..]);
    // }

    println!("Ok");
    // let contract = state.get_contract();

    // let _ = adder.add(&RustBigInt::from(7));
    // assert_eq!(RustBigInt::from(12), *adder.get_mut_sum());

    // let _ = adder.add(&RustBigInt::from(1));
    // assert_eq!(RustBigInt::from(13), *adder.get_mut_sum());
}
