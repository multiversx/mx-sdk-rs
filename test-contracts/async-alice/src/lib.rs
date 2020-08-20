
#![no_std]
#![allow(non_snake_case)]

imports!();

static CALEE_STORAGE_KEY: &[u8] = &[0u8; 32];
static CALLBACK_INFO_KEY: &[u8] = &[0x77u8; 32];
static SOME_ADDRESS:      [u8; 32] = [0xfeu8; 32];

#[elrond_wasm_derive::callable(PayMeProxy)]
pub trait PayMe {

    #[payable]
    fn payMe(&self, #[payment] _payment: BigUint, _arg1: i64);

    #[payable]
    #[callback(payCallback)]
    fn payMeWithResult(&self, #[payment] _payment: BigUint, _arg1: i64);
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMe {
    fn messageMe(&self, arg1: i64, arg2: &BigUint, arg3: &Vec<u8>, arg4: &Address);
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMeWithCallback {
    #[callback(messageCallback)]
    fn messageMe(&self, arg1: i64, arg2: BigUint, arg3: Vec<u8>, arg4: Address);
}

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Alice {

    #[init]
    fn init(&self, calee_address: Address) {
        self.storage_store_bytes32(CALEE_STORAGE_KEY, &calee_address.into());
    }

    #[payable]
    #[endpoint]
    fn forwardToOtherContract(&self, #[payment] payment: BigUint) {
        let calee_address: Address = self.storage_load_bytes32(CALEE_STORAGE_KEY).into();

        let target_contract = contract_proxy!(self, &calee_address, PayMe);
        target_contract.payMe(payment, 0x56);
    }

    #[payable]
    #[endpoint]
    fn forwardToOtherContractWithCallback(&self, #[payment] payment: BigUint) {
        let calee_address: Address = self.storage_load_bytes32(CALEE_STORAGE_KEY).into();

        let target_contract = contract_proxy!(self, &calee_address, PayMe);
        target_contract.payMeWithResult(payment, 0x56);
    }

    #[endpoint]
    fn messageOtherContract(&self) {
        let calee_address: Address = self.storage_load_bytes32(CALEE_STORAGE_KEY).into();

        let target_contract = contract_proxy!(self, &calee_address, MessageMe);
        target_contract.messageMe(0x01, &BigUint::from(0x02u64), &create_a_vec(), &SOME_ADDRESS.into());
    }

    #[endpoint]
    fn messageOtherContractWithCallback(&self) {
        let calee_address: Address = self.storage_load_bytes32(&CALEE_STORAGE_KEY).into();

        let target_contract = contract_proxy!(self, &calee_address, MessageMeWithCallback);
        target_contract.messageMe(0x01, BigUint::from(0x02u64), create_a_vec(), SOME_ADDRESS.into());
    }

    #[callback]
    fn payCallback(&self, call_result: AsyncCallResult<i64>) {
        match call_result {
            AsyncCallResult::Ok(cb_arg) => {
                self.storage_store_i64(&CALLBACK_INFO_KEY, cb_arg);
            },
            AsyncCallResult::Err(_) => {}
        }
    }

    #[callback]
    fn messageCallback(&self, _call_result: AsyncCallResult<()>) {
        self.storage_store_i64(&CALLBACK_INFO_KEY, 0x5555);
    }
}

fn create_a_vec() -> Vec<u8> {
    let mut res = Vec::with_capacity(3);
    res.push(3);
    res.push(3);
    res.push(3);
    res
}
