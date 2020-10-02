#![allow(unused_variables)] // for now

use adder::*;
use elrond_wasm_debug::*;

fn main() {
    let mut contract_map = ContractMap::<TxContext>::new();
    contract_map.register_contract(
        "file:../output/adder.wasm",
        Box::new(|context| Box::new(AdderImpl::new(context))));

    parse_execute_mandos("examples/adder/mandos/adder.scen.json", &contract_map);
    
    println!("Ok");
    
    // mock_ref.add_account(AccountData{
    //     address: ADDR1.into(),
    //     nonce: 0,
    //     balance: 0u32.into(),
    //     storage: HashMap::new(),
    //     contract: None,
    // });
    // let scenario = mandos::parse_scenario("mandos/adder.scen.json");
    // print!("{:?}", scenario);


    // tx 1: init
    

    // // tx 2: add!
    // let mut tx2 = TxData::new_call(
    //     b"add".to_vec(), 
    //     ADDR1.into(), 
    //     ADDR2.into());
    // tx2.add_arg(vec![2u8]);
    // let result2 = mock_ref.execute_tx(tx2);
    // assert_eq!(0, result2.result_status);
    // //result2.print();

    // // tx 3: getSum
    // let tx3 = TxData::new_call(
    //     "getSum", 
    //     ADDR1.into(), 
    //     ADDR2.into());
    // let result3 = mock_ref.execute_tx(tx3);
    // result3.print();

    // mock_ref.print_accounts();
}
