

use elrond_wasm::Address;
use elrond_wasm::StorageKey;

use crate::big_int::*;
use elrond_wasm::BigIntApi;
use elrond_wasm::ContractHookApi;

use alloc::vec::Vec;

const ADDRESS_LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;


extern {
    fn getOwner(resultOffset: *mut u8);
    fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;
    fn getNumArguments() -> i32;
    fn getArgumentLength(id: i32) -> i32;
    fn getArgument(id: i32, dstOffset: *mut u8) -> i32;
    fn getFunction(functionOffset: *const u8) -> i32;
    fn storageStore(keyOffset: *const u8, dataOffset: *const u8, dataLength: i32) -> i32;
    fn storageGetValueLength(keyOffset: *const u8) -> i32;
    fn storageLoad(keyOffset: *const u8, dataOffset: *mut u8) -> i32;

    fn transferValue(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32) -> i32;
    fn asyncCall(dstOffset: *const u8, valueOffset: *const u8, dataOffset: *const u8, length: i32);

    fn getCaller(resultOffset: *mut u8);
    fn callValue(resultOffset: *const u8) -> i32;
    fn writeLog(pointer: *const u8, length: i32, topicPtr: *const u8, numTopics: i32);
    fn finish(dataOffset: *const u8, length: i32);
    fn signalError(messageOffset: *const u8, messageLength: i32);

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
    
    fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);
    fn bigIntGetUnsignedArgument(arg_id: i32, dest: i32);
    fn bigIntGetSignedArgument(arg_id: i32, dest: i32);
    fn bigIntGetCallValue(dest: i32);
    fn bigIntFinish(bih: i32);

    fn int64getArgument(id: i32) -> i64;
    fn int64finish(value: i64);
    fn int64storageStore(keyOffset: *const u8, value: i64) -> i32;
    fn int64storageLoad(keyOffset: *const u8) -> i64;
    
    fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
    fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
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

    fn get_balance(&self, address: &Address) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntGetExternalBalance(address.as_ref().as_ptr(), result);
            ArwenBigInt {handle: result}
        }
    }
    
    fn storage_store(&self, key: &StorageKey, value: &Vec<u8>) {
        unsafe {
            storageStore(key.as_ref().as_ptr(), value.as_ptr(), value.len() as i32);
        }
    }

    fn storage_load(&self, key: &StorageKey) -> Vec<u8> {
         unsafe {
            let value_len = storageGetValueLength(key.as_ref().as_ptr()) as usize;
            let mut res = Vec::with_capacity(value_len);
            storageLoad(key.as_ref().as_ptr(), res.as_mut_ptr());
            res.set_len(value_len);
            res
        }
    }

    fn storage_store_bytes32(&self, key: &StorageKey, value: &[u8; 32]) {
        unsafe {
            storageStore(key.as_ref().as_ptr(), value.as_ptr(), 32);
        }
    }
    
    fn storage_load_bytes32(&self, key: &StorageKey) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            let len = storageLoad(key.as_ref().as_ptr(), res.as_mut_ptr());
            if len != 32 {
                let message = "32 bytes of data expected in storage at key";
                signalError(message.as_ptr(), message.len() as i32);
            }
            res
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
    fn storage_store_i64(&self, key: &StorageKey, value: i64) {
        unsafe {
            int64storageStore(key.as_ref().as_ptr(), value);
        }
    }
    
    #[inline]
    fn storage_load_i64(&self, key: &StorageKey) -> Option<i64> {
        unsafe{
            Some(int64storageLoad(key.as_ref().as_ptr()))
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

    fn send_tx(&self, to: &Address, amount: &ArwenBigInt, message: &str) {
        let mut amount_bytes32 = amount.to_bytes_big_endian_pad_right(32);
        amount_bytes32.reverse(); // we need little endian
        unsafe {
            transferValue(
                to.as_ref().as_ptr(),
                amount_bytes32.as_ptr(),
                message.as_ptr(),
                message.len() as i32
            );
        }
    }

    fn async_call(&self, to: &Address, amount: &ArwenBigInt, data: &str) {
        let mut amount_bytes32 = amount.to_bytes_big_endian_pad_right(32);
        amount_bytes32.reverse(); // we need little endian
        unsafe {
            asyncCall(
                to.as_ref().as_ptr(),
                amount_bytes32.as_ptr(),
                data.as_ptr(),
                data.len() as i32
            );
        }
    }

    #[inline]
    fn get_gas_left(&self) -> i64 {
        unsafe { getGasLeft() }
    }

    fn sha256(&self, data: &Vec<u8>) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
        }
    }

    fn keccak256(&self, data: &Vec<u8>) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
        }
    }
}

impl elrond_wasm::ContractIOApi<ArwenBigInt, ArwenBigUint> for ArwenApiImpl {

    #[inline]
    fn get_num_arguments(&self) -> i32 {
        unsafe { getNumArguments() }
    }

    fn check_not_payable(&self) -> bool {
        if &self.get_call_value_big_int() > &0.into() {
            self.signal_error("attempted to transfer funds via a non-payable function");
            return false;
        }
        return true;
    }

    fn get_argument_vec(&self, arg_index: i32) -> Vec<u8> {
        unsafe {
            let len = getArgumentLength(arg_index);
            let mut res = Vec::with_capacity(len as usize);
            res.set_len(len as usize);
            getArgument(arg_index, res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            let len = getArgument(arg_index, res.as_mut_ptr());
            if len != 32 {
                self.signal_error("32 bytes of data expected as argument value");
            }
            res
        }
    }
    
    #[inline]
    fn get_argument_big_int_unsigned(&self, arg_id: i32) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntGetUnsignedArgument(arg_id, result);
            ArwenBigUint {handle: result}
        }
    }

    #[inline]
    fn get_argument_big_int_signed(&self, arg_id: i32) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntGetSignedArgument(arg_id, result);
            ArwenBigInt {handle: result}
        }
    }

    #[inline]
    fn get_argument_i64(&self, arg_id: i32) -> i64 {
        unsafe { int64getArgument(arg_id) }
    }
    
    #[inline]
    fn finish_vec(&self, v: Vec<u8>) {
        unsafe {
            finish(v.as_ptr(), v.len() as i32);
        }
    }

    #[inline]
    fn finish_big_int_signed(&self, b: ArwenBigInt) {
        unsafe {
            bigIntFinish(b.handle);
        }
    }

    #[inline]
    fn finish_big_int_unsigned(&self, b: ArwenBigUint) {
        unsafe {
            bigIntFinish(b.handle);
        }
    }
    
    #[inline]
    fn finish_i64(&self, value: i64) {
        unsafe { int64finish(value); }
    }

    #[inline]
    fn signal_error_raw(&self, message_ptr: *const u8, message_len: usize) {
        unsafe { signalError(message_ptr, message_len as i32) }
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
}

impl Clone for ArwenApiImpl {
    #[inline]
    fn clone(&self) -> Self {
        ArwenApiImpl {}
    }
}
