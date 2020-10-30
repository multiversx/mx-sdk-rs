
#![no_std]
#![allow(non_snake_case)]

imports!();

static STORAGE_KEY_1: &[u8] = &[0x11u8; 32];
static STORAGE_KEY_2: &[u8] = &[0x22u8; 32];
static STORAGE_KEY_3: &[u8] = &[0x33u8; 32];
static STORAGE_KEY_4: &[u8] = &[0x44u8; 32];
static STORAGE_KEY_5: &[u8] = &[0x55u8; 32];
static LAST_PAY_KEY:  &[u8] = &[0xffu8; 32];

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Bob {

    #[init]
    fn init(&self) {
    }

    #[payable]
    #[endpoint]
    fn payMe(&self, #[payment] payment: BigUint, arg1: i64) {
        self.storage_store_big_uint(LAST_PAY_KEY, &payment);
        self.storage_store_i64(STORAGE_KEY_1, arg1);
    }

    #[payable]
    #[endpoint]
    fn payMeWithResult(&self, #[payment] payment: BigUint, arg1: i64) -> i64 {
        self.payMe(payment, arg1);
        0x7777
    }

    #[endpoint]
    fn messageMe(&self, arg1: i64, arg2: &BigUint, arg3: Vec<u8>, arg4: Address) {
        self.storage_store_i64(STORAGE_KEY_2, arg1);
        self.storage_store_big_uint(STORAGE_KEY_3, arg2);
        self.storage_store_slice_u8(STORAGE_KEY_4, &arg3);
        self.storage_store_bytes32(STORAGE_KEY_5, &arg4.into());
    }
}
