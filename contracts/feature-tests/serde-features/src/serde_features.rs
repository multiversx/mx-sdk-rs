#![no_std]

use multiversx_sc::imports::*;
use serde::{Deserialize, Serialize};

// #[type_abi]
// #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem)]
#[derive(Serialize, Deserialize)]
pub struct SerdeStruct {
    pub value1: u32,
    pub value2: u32,
}

const BUFFER_SIZE: usize = 200;

#[multiversx_sc::contract]
pub trait SerdeFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn out_value_1(&self) -> ManagedBuffer {
        let s = SerdeStruct {
            value1: 10,
            value2: 20,
        };

        multiversx_sc::serde::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
        // serde_json_core::to_slice(value, buf)
        // let v = serde_json_core::to_vec(&s).unwrap_or_else(|_| sc_panic!("serialization failed"));
        // v.to_vec()
        // serde_json::to_vec(&s).unwrap_or_else(|_| sc_panic!("serialization failed"))
    }

    #[endpoint]
    fn inc_serde_1(&self, json: ManagedBuffer) -> ManagedBuffer {
        // let (mut s, _) = serde_json_core::from_slice::<SerdeStruct>(input)
        //     .unwrap_or_else(|_| sc_panic!("deserialization failed"));
        let mut buf = [0u8; BUFFER_SIZE];
        let slice = json.load_to_byte_array(&mut buf);
        let (mut s, _) = serde_json_core::from_slice::<SerdeStruct>(slice)
            .unwrap_or_else(|_| sc_panic!("deserialization failed"));
        s.value1 += 1;
        s.value2 += 1;
        multiversx_sc::serde::to_buffered_json::<_, _, BUFFER_SIZE>(&s)
    }
}
