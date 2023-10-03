#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[allow(unused)]
extern "C" {

    fn createAccount(
        addressHandle: i32,
        nonce: i64,
        balanceHandle: i32,
    );
    
    fn registerNewAddress(
        ownerHandle: i32,
        nonce: i64,
        newAddressHandle: i32,
    );

    fn deployContract(
        ownerHandle: i32,
        gasLimit: i64,
        valueHandle: i32,
        codePathHandle: i32,
        argumentsHandle: i32,
        resultAddressHandle: i32,
    );

    fn setStorage(
        addressHandle: i32,
        keyHandle: i32,
        valueHandle: i32,
    );

    fn getStorage(
        addressHandle: i32,
        keyHandle: i32,
        dstHandle: i32,
    );

    fn assumeBool(p: bool);
    fn assertBool(p: bool);

    fn startPrank(addressHandle: i32);
    fn stopPrank();

    fn setBlockTimestamp(timestamp: i64);

    fn setExternalBalance(
        addressHandle: i32,
        valueHandle: i32,
    );

    fn setESDTExternalBalance(
        addressHandle: i32,
        tokenIdHandle: i32,
        valueHandle: i32,
    );
}


#[allow(unused)]
pub fn create_account<M: ManagedTypeApi>(
    address: &ManagedAddress<M>,
    nonce: u64,
    balance: &BigUint<M>,
) {
    unsafe {
        createAccount(
            address.get_raw_handle(),
            nonce as i64,
            balance.get_raw_handle(),
        );
    }
}

#[allow(unused)]
pub fn register_new_address<M: ManagedTypeApi>(
    owner: &ManagedAddress<M>,
    nonce: u64,
    new_address: &ManagedAddress<M>,
) {
    unsafe {
        registerNewAddress(
            owner.get_raw_handle(),
            nonce as i64,
            new_address.get_raw_handle(),
        );
    }
}

// Deploy a contract whose code was previously fetched using "fetchWasmSource" in Mandos.
#[allow(unused)]
pub fn deploy_contract<M: ManagedTypeApi>(
    owner: &ManagedAddress<M>,
    gas_limit: u64,
    value: &BigUint<M>,
    code_path: &ManagedBuffer<M>,
    arguments: &ManagedArgBuffer<M>,
) -> ManagedAddress<M> {
    unsafe {
        let mut dest = ManagedAddress::zero();
        
        deployContract(
            owner.get_raw_handle(),
            gas_limit as i64,
            value.get_raw_handle(),
            code_path.get_raw_handle(),
            arguments.get_raw_handle(),
            dest.get_raw_handle(),
        );

        dest
    }

}

// Set storage of any account
#[allow(unused)]
pub fn set_storage<M: ManagedTypeApi>(
    address: &ManagedAddress<M>,
    key: &ManagedBuffer<M>,
    value: &ManagedBuffer<M>,
) {
    unsafe {
        setStorage(
            address.get_raw_handle(),
            key.get_raw_handle(),
            value.get_raw_handle(),
        );
    }
}


// Get storage of any account
#[allow(unused)]
pub fn get_storage<M: ManagedTypeApi>(
    address: &ManagedAddress<M>,
    key: &ManagedBuffer<M>,
) -> ManagedBuffer<M> {
    unsafe {
        let mut dest = ManagedBuffer::new();
        
        getStorage(
            address.get_raw_handle(),
            key.get_raw_handle(),
            dest.get_raw_handle(),
        );

        dest
    }
}


// Start a prank: set the caller address for contract calls until stop_prank 
#[allow(unused)]
pub fn start_prank<M: ManagedTypeApi>(address: &ManagedAddress<M>) {
    unsafe {
        startPrank(address.get_raw_handle());
    }
}

// Stop a prank: reset the caller address
#[allow(unused)]
pub fn stop_prank() {
    unsafe {
        stopPrank();
    }
}

#[allow(unused)]
pub fn assume(p: bool) {
    unsafe {
        assumeBool(p);
    }
}

#[allow(unused)]
pub fn assert(p: bool) {
    unsafe {
        assertBool(p);
    }
}

#[allow(unused)]
pub fn set_block_timestamp(timestamp: u64) {
    unsafe {
        setBlockTimestamp(timestamp as i64);
    }
}

#[allow(unused)]
pub fn set_balance<M: ManagedTypeApi>(
    address: &ManagedAddress<M>,
    value: &BigUint<M>,
) {
    unsafe {
        setExternalBalance(
            address.get_raw_handle(),
            value.get_raw_handle(),
        );
    }
}


#[allow(unused)]
pub fn set_esdt_balance<M: ManagedTypeApi>(
    address: &ManagedAddress<M>,
    token_id: &TokenIdentifier<M>,
    value: &BigUint<M>,
) {
    unsafe {
        setESDTExternalBalance(
            address.get_raw_handle(),
            token_id.get_raw_handle(),
            value.get_raw_handle(),
        );
    }
}
