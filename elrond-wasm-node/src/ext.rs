

use elrond_wasm::Address;
use elrond_wasm::StorageKey;

use crate::big_int::*;
use elrond_wasm::BigIntApi;
use elrond_wasm::ContractHookApi;

//use alloc::boxed::Box;
//use alloc::vec::Vec;

const ADDRESS_LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;


extern {
    fn getOwner(resultOffset: *mut u8);
    fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;
    fn transferValue(gasLimit: i64, dstOffset: *mut u8, sndOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32) -> i32;
    fn getNumArguments() -> i32;
    fn getArgument(id: i32, dstOffset: *mut u8) -> i32;
    fn getFunction(functionOffset: *const u8) -> i32;
    fn storageStore(keyOffset: *const u8, dataOffset: *const u8, dataLength: i32);
    fn storageLoad(keyOffset: *const u8, dataOffset: *mut u8) -> i32;

    fn getCaller(resultOffset: *mut u8);
    fn callValue(resultOffset: *const u8) -> i32;
    fn writeLog(pointer: *const u8, length: i32, topicPtr: *const u8, numTopics: i32);
    fn returnData(dataOffset: *const u8, length: i32);
    fn signalError() -> !;

    fn getGasLeft() -> i64;
    fn getBlockTimestamp() -> i64;
    fn getBlockNonce() -> i64;
    fn getBlockRound() -> i64;
    fn getBlockEpoch() -> i64;
    fn getBlockRandomSeed(resultOffset: *mut u8);
    fn getStateRootHash(resultOffset: *mut u8);
    fn getPrevBlockTimestamp() -> i64;
    fn getPrevBlockNonce() -> i64;
    fn getPrevBlockRound() -> i64;
    fn getPrevBlockEpoch() -> i64;
    fn getPrevBlockRandomSeed(resultOffset: *const u8);


    fn bigIntNew(value: i64) -> i32;

    fn bigIntByteLength(x: i32) -> i32;
    fn bigIntGetBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
    fn bigIntCmp(x: i32, y: i32) -> i32;

    fn bigIntStorageStore(key_ptr: *const u8, source: i32) -> i32;
    fn bigIntStorageLoad(key_ptr: *const u8, destination: i32) -> i32;
    
    fn bigIntGetArgument(arg_id: i32, dest: i32);
    fn bigIntGetCallValue(dest: i32);
    fn bigIntFinish(bih: i32);

    fn int64getArgument(id: i32) -> i64;
    //fn int64storageStore(keyOffset: *const u8, value: i64) -> i32;
    //fn int64storageLoad(keyOffset: *const u8) -> i64;
    fn int64finish(value: i64);
}

pub struct ArwenApiImpl {}
impl elrond_wasm::ContractHookApi<ArwenBigInt> for ArwenApiImpl {
    #[inline]
    fn get_owner(&self) -> Address {
        unsafe {
            let mut res = [0u8; 32];
            getOwner(res.as_mut_ptr());
            res.into()
        }
    }

    #[inline]
    fn get_caller(&self) -> Address {
        unsafe {
            let mut res = [0u8; 32];
            getCaller(res.as_mut_ptr());
            res.into()
        }
    }

    #[inline]
    fn signal_error(&self) {
        unsafe { signalError() }
    }

    fn write_log(&self, topics: &[[u8;32]], data: &[u8]) {
        let mut topics_raw = [0u8; TOPIC_LENGTH * 10]; // hopefully we never have more than 10 topics
        for i in 0..topics.len() {
            topics_raw[TOPIC_LENGTH*i..TOPIC_LENGTH*(i+1)].copy_from_slice(&topics[i]);
        }
        unsafe {
            writeLog(data.as_ptr(), data.len() as i32, topics_raw.as_ptr(), topics.len() as i32);
        }
    }

    #[inline]
    fn storage_store_big_int(&self, key: &StorageKey, value: &ArwenBigInt) {
        unsafe {
            bigIntStorageStore(key.as_ref().as_ptr(), value.handle);
        }
    }

    #[inline]
    fn storage_load_big_int(&self, key: &StorageKey) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntStorageLoad(key.as_ref().as_ptr(), result);
            ArwenBigInt {handle: result}
        }
    }

    #[inline]
    fn get_call_value_big_int(&self) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntGetCallValue(result);
            ArwenBigInt {handle: result}
        }
    }
}

impl elrond_wasm::ContractIOApi<ArwenBigInt> for ArwenApiImpl {

    fn check_num_arguments(&self, expected: i32) -> bool {
        let nr_arg = unsafe { getNumArguments() };
        if nr_arg != expected {
            self.signal_error();
            return false;
        }
        return true;
    }

    fn check_not_payable(&self) -> bool {
        if ArwenBigInt::compare(&self.get_call_value_big_int(), &ArwenBigInt::from(0)) > 0 {
            self.signal_error();
            return false;
        }
        return true;
    }

    #[inline]
    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            getArgument(arg_index, res.as_mut_ptr());
            res
        }
    }
    
    #[inline]
    fn get_argument_address(&self, arg_index: i32) -> Address {
        unsafe {
            let mut res = [0u8; 32];
            getArgument(arg_index, res.as_mut_ptr());
            res.into()
        }
    }
    
    #[inline]
    fn get_argument_big_int(&self, arg_id: i32) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntGetArgument(arg_id, result);
            ArwenBigInt {handle: result}
        }
    }

    #[inline]
    fn get_argument_i64(&self, arg_id: i32) -> i64 {
        unsafe { int64getArgument(arg_id) }
    }

    #[inline]
    fn finish_big_int(&self, b: ArwenBigInt) {
        unsafe {
            bigIntFinish(b.handle);
        }
    }
    
    #[inline]
    fn finish_i64(&self, value: i64) {
        unsafe { int64finish(value); }
    }
}

impl Clone for ArwenApiImpl {
    #[inline]
    fn clone(&self) -> Self {
        ArwenApiImpl {}
    }
}
