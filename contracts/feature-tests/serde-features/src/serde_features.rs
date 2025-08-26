#![no_std]

use multiversx_sc::imports::*;
use serde::{Deserialize, Serialize};

const BUFFER_SIZE: usize = 200;

// #[type_abi]
// #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem)]
#[derive(Serialize, Deserialize)]
pub struct SerdeStruct1 {
    pub value1: u32,
    pub value2: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SerdeStruct2 {
    // pub v: Vec<u8>,
    pub big_int: num_bigint::BigInt,
}

#[derive(Serialize, Deserialize)]
pub struct ManagedSerdeStruct<M: ManagedTypeApi> {
    mb: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait SerdeFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn out_value_1(&self) -> ManagedBuffer {
        let s = SerdeStruct1 {
            value1: 10,
            value2: 20,
        };

        multiversx_sc::serde_util::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }

    #[endpoint]
    fn out_value_2(&self) -> ManagedBuffer {
        let s = ManagedSerdeStruct::<Self::Api> { mb: "abc".into() };

        multiversx_sc::serde_util::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }

    #[endpoint]
    fn inc_serde_1(&self, json: ManagedBuffer) -> ManagedBuffer {
        let mut buf = [0u8; BUFFER_SIZE];
        let slice = json.load_to_byte_array(&mut buf);
        let (mut s, _) = serde_json_core::from_slice::<SerdeStruct1>(slice)
            .unwrap_or_else(|_| sc_panic!("deserialization failed"));
        s.value1 += 1;
        s.value2 += 1;
        multiversx_sc::serde_util::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }

    #[endpoint]
    fn inc_serde_2(&self, json: ManagedBuffer) -> ManagedBuffer {
        let mut buf = [0u8; BUFFER_SIZE];
        let slice = json.load_to_byte_array(&mut buf);
        let (mut s, _) = serde_json_core::from_slice::<ManagedSerdeStruct<Self::Api>>(slice)
            .unwrap_or_else(|_| sc_panic!("deserialization failed"));
        s.mb.append_bytes(b"def");
        multiversx_sc::serde_util::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }
}
