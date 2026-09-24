#![no_std]

mod adder_proxy;

multiversx_sc::imports!();

static INIT_SUM: u32 = 5u32;

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

        self.test_raw()
            .create_account(&owner, 1, &BigUint::from(0u64));

        // register an address for the contract to be deployed
        let adder = ManagedAddress::from(b"adder___________________________");
        self.test_raw().register_new_address(&owner, 1, &adder);

        let adder = self
            .tx()
            .from(&owner)
            .typed(adder_proxy::AdderProxy)
            .init(INIT_SUM)
            .code_path(code_path)
            .gas(5000000000000)
            .returns(ReturnsNewManagedAddress)
            .test_deploy();

        // save the deployed contract's address
        self.adder_address().set(&adder);

        // check the initial sum value
        let sum: BigUint = self.storage_raw().read_from_address(&adder, "sum");
        self.test_raw().assert(sum == INIT_SUM);
    }

    // Make a call from 'owner' to 'adder' and check the sum value
    #[endpoint(test_call_add)]
    fn test_call_add(&self, value: BigUint) {
        self.test_raw().assume(value <= 100u32);

        let adder = self.adder_address().get();

        self.call_add(&value);

        // check the sum value
        let sum: BigUint = self.storage_raw().read_from_address(&adder, "sum");
        self.test_raw().assert(sum == (value + INIT_SUM));
    }

    #[endpoint(test_call_add_twice)]
    fn test_call_add_twice(&self, value1: BigUint, value2: BigUint) {
        self.test_raw().assume(value1 <= 100u32);
        self.test_raw().assume(value2 <= 100u32);

        let adder = self.adder_address().get();

        self.call_add(&value1);
        self.call_add(&value2);

        // check the sum value
        let sum: BigUint = self.storage_raw().read_from_address(&adder, "sum");
        self.test_raw().assert(sum == (value1 + value2 + INIT_SUM));
    }

    fn call_add(&self, value: &BigUint) {
        let owner = self.owner_address().get();
        let adder = self.adder_address().get();

        // start a prank and call 'adder' from 'owner'
        self.tx()
            .from(owner)
            .to(adder)
            .typed(adder_proxy::AdderProxy)
            .add(value)
            .gas(5000000)
            .test_call();
    }
}
