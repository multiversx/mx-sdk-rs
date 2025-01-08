#![no_std]

multiversx_sc::imports!();

use hex_literal::hex;

pub mod message_me_proxy;
pub mod pay_me_proxy;

static HARDCODED_ADDRESS: [u8; 32] =
    hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

#[multiversx_sc::contract]
pub trait ProxyTestFirst {
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
    fn deploy_second_contract(&self, code: ManagedBuffer) -> i32 {
        let payment = self.call_value().egld();

        let (address, init_result) = self
            .tx()
            .typed(message_me_proxy::MessageMeProxy)
            .init(123)
            .code(code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewManagedAddress)
            .returns(ReturnsResult)
            .egld(payment)
            .sync_call();

        self.set_other_contract(&address);
        init_result + 1
    }

    #[payable("EGLD")]
    #[endpoint(upgradeSecondContract)]
    fn upgrade_second_contract(&self, code: ManagedBuffer) {
        let payment = self.call_value().egld();
        let other_contract = self.get_other_contract();

        self.tx()
            .to(other_contract)
            .typed(pay_me_proxy::PayMeProxy)
            .upgrade()
            .argument(&456)
            .egld(payment)
            .code(code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .upgrade_async_call_and_exit();
    }

    #[payable("EGLD")]
    #[endpoint(forwardToOtherContract)]
    fn forward_to_other_contract(&self) {
        let payment = self.call_value().egld();
        let other_contract = self.get_other_contract();
        self.tx()
            .to(&other_contract)
            .typed(pay_me_proxy::PayMeProxy)
            .pay_me(0x56)
            .egld(payment)
            .async_call_and_exit();
    }

    #[payable("EGLD")]
    #[endpoint(forwardToOtherContractWithCallback)]
    fn forward_to_other_contract_with_callback(&self) {
        let payment = self.call_value().egld();
        let other_contract = self.get_other_contract();
        self.tx()
            .to(&other_contract)
            .typed(pay_me_proxy::PayMeProxy)
            .pay_me_with_result(0x56)
            .egld(payment)
            .callback(self.callbacks().pay_callback())
            .async_call_and_exit();
    }

    #[endpoint(messageOtherContract)]
    fn message_other_contract(&self) {
        let other_contract = self.get_other_contract();
        self.tx()
            .to(&other_contract)
            .typed(message_me_proxy::MessageMeProxy)
            .message_me(
                0x01,
                &BigUint::from(2u32),
                [3u8; 3].to_vec(),
                &ManagedAddress::from(&HARDCODED_ADDRESS),
            )
            .async_call_and_exit()
    }

    #[endpoint(messageOtherContractWithCallback)]
    fn message_other_contract_with_callback(&self) {
        let other_contract = self.get_other_contract();
        self.tx()
            .to(&other_contract)
            .typed(message_me_proxy::MessageMeProxy)
            .message_me(
                0x01,
                &BigUint::from(2u32),
                [3u8; 3].to_vec(),
                &ManagedAddress::from(&HARDCODED_ADDRESS),
            )
            .callback(self.callbacks().message_callback())
            .async_call_and_exit()
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
