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

#[multiversx_sc::contract]
pub trait SerdeFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn out_value_1(&self) -> Vec<u8> {
        let s = SerdeStruct {
            value1: 10,
            value2: 20,
        };
        // serde_json_core::to_slice(value, buf)
        serde_json::to_vec(&s).unwrap_or_else(|_| sc_panic!("serialization failed"))
    }

    #[endpoint]
    fn out_value_1_pretty(&self) -> Vec<u8> {
        let s = SerdeStruct {
            value1: 10,
            value2: 20,
        };
        serde_json::to_vec_pretty(&s).unwrap_or_else(|_| sc_panic!("serialization failed"))
    }

    #[endpoint]
    fn inc_serde_1(&self, input: &[u8]) -> Vec<u8> {
        let (mut s, _) = serde_json_core::from_slice::<SerdeStruct>(input)
            .unwrap_or_else(|_| sc_panic!("deserialization failed"));
        s.value1 += 1;
        s.value2 += 1;
        serde_json::to_vec(&s).unwrap_or_else(|_| sc_panic!("serialization failed"))
    }
}
