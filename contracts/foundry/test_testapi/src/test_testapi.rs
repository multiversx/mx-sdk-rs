#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait TestTestapi {
    #[init]
    fn init(&self) {
        let alice = ManagedAddress::from(b"alice___________________________");
        self.test_raw()
            .create_account(&alice, 1, &BigUint::from(0u64));

        self.test_set_balance(&alice);
        self.test_set_esdt_balance(&alice);
        self.test_set_timestamp();
        self.test_set_get_storage(&alice);
    }

    fn test_set_balance(&self, addr: &ManagedAddress) {
        // Given
        let value = BigUint::from(100000000u64);

        // When
        self.test_raw().set_balance(addr, &value);

        // Expect
        let actual = self.blockchain().get_balance(addr);

        require!(
            value == actual,
            "Actual balance does not match the given value"
        );
    }

    fn test_set_esdt_balance(&self, addr: &ManagedAddress) {
        // Given
        let value = BigUint::from(100000000u64);
        let token = TokenIdentifier::from("MY_ESDT_TOKEN");

        // When
        self.test_raw().set_esdt_balance(addr, &token, &value);

        // Expect
        let actual = self.blockchain().get_esdt_balance(addr, &token, 0u64);

        require!(
            value == actual,
            "Actual esdt balance does not match the given value"
        );
    }

    fn test_set_timestamp(&self) {
        // Given
        let value = 1234567890u64;

        // When
        self.test_raw().set_block_timestamp(value);

        // Expect
        require!(
            value == self.blockchain().get_block_timestamp(),
            "Actual timestamp does not match the given value"
        );
    }

    fn test_set_get_storage(&self, addr: &ManagedAddress) {
        // Given
        let key = ManagedBuffer::from(b"a_storage_key");
        let value = ManagedBuffer::from(b"a storage value");

        // When
        self.test_raw().set_storage(addr, &key, &value);

        // Expect
        let actual = self.test_raw().get_storage(addr, &key);
        require!(
            actual == value,
            "Actual storage does not match the given value"
        );
    }
}
