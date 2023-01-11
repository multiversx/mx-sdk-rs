#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait ProxyTestSecond {
    #[storage_set("last_payment")]
    fn set_last_payment(&self, last_payment: &BigUint);

    #[storage_set("init_arg")]
    fn set_init_arg(&self, init_arg: i32);

    #[storage_set("pay_me_arg")]
    fn set_pay_me_arg(&self, arg: i64);

    #[storage_set("message_me_1")]
    fn set_message_me_1(&self, m1: i64);

    #[storage_set("message_me_2")]
    fn set_message_me_2(&self, s2: &BigUint);

    #[storage_set("message_me_3")]
    fn set_message_me_3(&self, s3: &BoxedBytes);

    #[storage_set("message_me_4")]
    fn set_message_me_4(&self, s4: &ManagedAddress);

    #[init]
    #[payable("EGLD")]
    fn init(&self, init_arg: i32) -> i32 {
        let payment = self.call_value().egld_value();
        self.set_last_payment(&payment);
        self.set_init_arg(init_arg);
        init_arg + 1
    }

    #[payable("EGLD")]
    #[endpoint(payMe)]
    fn pay_me(&self, arg1: i64) {
        let payment = self.call_value().egld_value();
        self.set_last_payment(&payment);
        self.set_pay_me_arg(arg1);
    }

    #[payable("EGLD")]
    #[endpoint(payMeWithResult)]
    fn pay_me_with_result_endpoint(&self, arg1: i64) -> i64 {
        let payment = self.call_value().egld_value();
        self.set_last_payment(&payment);
        self.set_pay_me_arg(arg1);

        0x7777
    }

    #[endpoint(messageMe)]
    fn message_me(&self, arg1: i64, arg2: &BigUint, arg3: &BoxedBytes, arg4: &ManagedAddress) {
        self.set_message_me_1(arg1);
        self.set_message_me_2(arg2);
        self.set_message_me_3(arg3);
        self.set_message_me_4(arg4);
    }
}
