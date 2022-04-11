#![no_std]

elrond_wasm::imports!();

use hex_literal::hex;

static HARDCODED_ADDRESS: [u8; 32] =
    hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

mod pay_me_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait PayMe {
        #[payable("EGLD")]
        #[endpoint(payMe)]
        fn pay_me(&self, #[payment] payment: BigUint, arg1: i64);

        #[payable("EGLD")]
        #[endpoint(payMeWithResult)]
        fn pay_me_with_result(&self, #[payment] payment: BigUint, arg1: i64);
    }
}

mod message_me_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait MessageMe {
        #[init]
        #[payable("EGLD")]
        fn init(&self, #[payment] payment: BigUint, init_arg: i32) -> i32;

        #[endpoint(messageMe)]
        fn message_me(&self, arg1: i64, arg2: &BigUint, arg3: Vec<u8>, arg4: &ManagedAddress);
    }
}

#[elrond_wasm::contract]
pub trait ProxyTestFirst {
    #[proxy]
    fn pay_me_proxy(&self) -> pay_me_proxy::Proxy<Self::Api>;

    #[proxy]
    fn message_me_proxy(&self) -> message_me_proxy::Proxy<Self::Api>;

    #[storage_get("other_contract")]
    fn get_other_contract(&self) -> ManagedAddress;

    #[storage_set("other_contract")]
    fn set_other_contract(&self, other_contract: &ManagedAddress);

    #[storage_set("callback_info")]
    fn set_callback_info(&self, callback_info: i64);

    #[init]
    fn init(&self, other_contract_addr: &ManagedAddress) {
        self.set_other_contract(other_contract_addr);
    }

    #[payable("EGLD")]
    #[endpoint(deploySecondContract)]
    fn deploy_second_contract(&self, #[payment] payment: BigUint, code: ManagedBuffer) -> i32 {
        let (address, init_result) = self
            .message_me_proxy()
            .init(payment, 123)
            .deploy_contract::<i32>(&code, CodeMetadata::DEFAULT);
        self.set_other_contract(&address);
        init_result + 1
    }

    #[payable("EGLD")]
    #[endpoint(upgradeSecondContract)]
    fn upgrade_second_contract(&self, #[payment] payment: BigUint, code: ManagedBuffer) {
        let other_contract = self.get_other_contract();

        self.message_me_proxy()
            .contract(other_contract)
            .init(payment, 456)
            .upgrade_contract(&code, CodeMetadata::DEFAULT);
    }

    #[payable("EGLD")]
    #[endpoint(forwardToOtherContract)]
    fn forward_to_other_contract(&self, #[payment] payment: BigUint) {
        let other_contract = self.get_other_contract();
        self.pay_me_proxy()
            .contract(other_contract)
            .pay_me(payment, 0x56)
            .async_call()
            .call_and_exit()
    }

    #[payable("EGLD")]
    #[endpoint(forwardToOtherContractWithCallback)]
    fn forward_to_other_contract_with_callback(&self, #[payment] payment: BigUint) {
        let other_contract = self.get_other_contract();
        self.pay_me_proxy()
            .contract(other_contract)
            .pay_me_with_result(payment, 0x56)
            .async_call()
            .with_callback(self.callbacks().pay_callback())
            .call_and_exit()
    }

    #[endpoint(messageOtherContract)]
    fn message_other_contract(&self) {
        let other_contract = self.get_other_contract();
        self.message_me_proxy()
            .contract(other_contract)
            .message_me(
                0x01,
                &BigUint::from(2u32),
                [3u8; 3].to_vec(),
                &ManagedAddress::from(&HARDCODED_ADDRESS),
            )
            .async_call()
            .call_and_exit()
    }

    #[endpoint(messageOtherContractWithCallback)]
    fn message_other_contract_with_callback(&self) {
        let other_contract = self.get_other_contract();
        self.message_me_proxy()
            .contract(other_contract)
            .message_me(
                0x01,
                &BigUint::from(2u32),
                [3u8; 3].to_vec(),
                &ManagedAddress::from(&HARDCODED_ADDRESS),
            )
            .async_call()
            .with_callback(self.callbacks().message_callback())
            .call_and_exit()
    }

    #[callback(payCallback)] // although uncommon, custom callback names are possible
    fn pay_callback(&self, #[call_result] call_result: ManagedAsyncCallResult<i64>) {
        match call_result {
            ManagedAsyncCallResult::Ok(cb_arg) => {
                self.set_callback_info(cb_arg);
            },
            ManagedAsyncCallResult::Err(_) => {},
        }
    }

    #[callback]
    fn message_callback(&self, #[call_result] _call_result: ManagedAsyncCallResult<()>) {
        self.set_callback_info(0x5555);
    }
}
