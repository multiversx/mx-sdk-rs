#![no_std]

use testapi;

multiversx_sc::imports!();

static INIT_SUM : u32 = 5u32;
#[multiversx_sc::contract]
pub trait TestAdder {

    #[storage_mapper("ownerAddress")]
    fn owner_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("adderAddress")]
    fn adder_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Create the owner account and deploy adder
    #[init]
    fn init(&self, code_path: ManagedBuffer) {
        
        // create the owner account
        let owner = ManagedAddress::from(b"owner___________________________");
        self.owner_address().set(&owner);
        
        testapi::create_account(&owner, 1, &BigUint::from(0u64));
        
        // register an address for the contract to be deployed
        let adder = ManagedAddress::from(b"adder___________________________");
        testapi::register_new_address(&owner, 1, &adder, );

        // deploy the adder contract
        let mut adder_init_args = ManagedArgBuffer::new();
        adder_init_args.push_arg(INIT_SUM); // initial sum

        // deploy a contract from `owner`
        let adder = testapi::deploy_contract(
                &owner,
                5000000000000,
                &BigUint::zero(),
                &code_path,
                &adder_init_args,
            );

        // save the deployed contract's address
        self.adder_address().set(&adder);

        // check the initial sum value
        let sum_as_bytes = testapi::get_storage(&adder, &ManagedBuffer::from(b"sum")); 
        let sum = BigUint::from(sum_as_bytes);
        testapi::assert( sum == INIT_SUM );

    }

    // Make a call from 'owner' to 'adder' and check the sum value
    #[endpoint(test_call_add)]
    fn test_call_add(&self, value: BigUint) {

        testapi::assume(value <= 100u32);

        let adder = self.adder_address().get();

        self.call_add(&value);

        // check the sum value
        let sum_as_bytes = testapi::get_storage(&adder, &ManagedBuffer::from(b"sum")); 
        let sum = BigUint::from(sum_as_bytes);
        testapi::assert( sum == (value + INIT_SUM) );

    }

    #[endpoint(test_call_add_twice)]
    fn test_call_add_twice(&self, value1: BigUint, value2: BigUint) {

        testapi::assume(value1 <= 100u32);
        testapi::assume(value2 <= 100u32);

        let adder = self.adder_address().get();

        self.call_add(&value1);
        self.call_add(&value2);

        // check the sum value
        let sum_as_bytes = testapi::get_storage(&adder, &ManagedBuffer::from(b"sum")); 
        let sum = BigUint::from(sum_as_bytes);
        testapi::assert( sum == (value1 + value2 + INIT_SUM) );

    }

    fn call_add(&self, value: &BigUint) {
        let owner = self.owner_address().get();
        let adder = self.adder_address().get();

        let mut adder_init_args = ManagedArgBuffer::new();
        adder_init_args.push_arg(&value); // initial sum

        // start a prank and call 'adder' from 'owner'
        testapi::start_prank(&owner);
        let res = self.send_raw().direct_egld_execute(
            &adder, 
            &BigUint::from(0u32), 
            5000000, 
            &ManagedBuffer::from(b"add"),
            &adder_init_args,
        );
        testapi::stop_prank();

        match res {
            Result::Err(_) => panic!("call failed"),
            Result::Ok(_) => ()
        };

    }
}
