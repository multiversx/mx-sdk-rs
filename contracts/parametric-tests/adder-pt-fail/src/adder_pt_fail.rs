#![no_std]

mod adder_proxy;

multiversx_sc::imports!();

static INIT_SUM: u32 = 5u32;

/// The single value of `sum` for which `test_rare_failure` fails.
static RARE_SUM: u32 = 123_456_789u32;

/// Parametric tests that are expected to FAIL.
///
/// They exist to check the tooling, not the Adder contract:
/// a fuzzer or prover that reports these as passing is broken.
#[multiversx_sc::contract]
pub trait TestAdderFail {
    #[storage_mapper("ownerAddress")]
    fn owner_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("adderAddress")]
    fn adder_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Create the owner account and deploy adder. Same setup as in `adder-pt`.
    #[init]
    fn init(&self, code_path: ManagedBuffer) {
        let owner = ManagedAddress::from(b"owner___________________________");
        self.owner_address().set(&owner);

        self.test_raw()
            .create_account(&owner, 1, &BigUint::from(0u64));

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

        self.adder_address().set(&adder);
    }

    /// Control: correct property, expected to PASS.
    /// Shows that a failure reported for the other tests is not caused by the setup.
    #[endpoint(test_control_passes)]
    fn test_control_passes(&self, value: BigUint) {
        self.test_raw().assume(value <= 100u32);

        self.call_add(&value);

        self.test_raw().assert(self.adder_sum() == (value + INIT_SUM));
    }

    /// Expected to FAIL for every input: the expected sum is off by one.
    /// Any fuzzing run should find this immediately.
    #[endpoint(test_wrong_sum)]
    fn test_wrong_sum(&self, value: BigUint) {
        self.test_raw().assume(value <= 100u32);

        self.call_add(&value);

        self.test_raw()
            .assert(self.adder_sum() == (value + INIT_SUM + 1u32));
    }

    /// Expected to FAIL for exactly one input out of all non-negative integers:
    /// `value == RARE_SUM - INIT_SUM`.
    /// Fuzzing is very unlikely to find it; symbolic execution must.
    #[endpoint(test_rare_failure)]
    fn test_rare_failure(&self, value: BigUint) {
        self.call_add(&value);

        self.test_raw().assert(self.adder_sum() != RARE_SUM);
    }

    fn adder_sum(&self) -> BigUint {
        let adder = self.adder_address().get();
        self.storage_raw().read_from_address(&adder, "sum")
    }

    fn call_add(&self, value: &BigUint) {
        let owner = self.owner_address().get();
        let adder = self.adder_address().get();

        self.tx()
            .from(owner)
            .to(adder)
            .typed(adder_proxy::AdderProxy)
            .add(value)
            .gas(5000000)
            .test_call();
    }
}
