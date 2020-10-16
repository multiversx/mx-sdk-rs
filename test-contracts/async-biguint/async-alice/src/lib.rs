
#![no_std]
#![allow(non_snake_case)]

imports!();

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMe {
    fn messageMe(&self, value: BigUint);
}

#[elrond_wasm_derive::contract(AliceImpl)]
pub trait Alice {

    #[init]
    fn init(&self, calee_address: Address) {
        self.set_callee(&calee_address);
    }

    #[endpoint]
    fn messageOtherContract(&self) {
        let calee_address = self.get_callee();
        
        let value = BigUint::from(0x64u32);

        let target_contract = contract_proxy!(self, &calee_address, MessageMe);
        target_contract.messageMe(value);
    }

    #[storage_set("callee")]
    fn set_callee(&self, address: &Address);

    #[view]
    #[storage_get("callee")]
    fn get_callee(&self) -> Address;
}
