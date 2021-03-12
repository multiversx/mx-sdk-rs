use super::{ArwenBigInt, ArwenBigUint};
use crate::ArwenApiImpl;
use elrond_wasm::api::ContractHookApi;
use elrond_wasm::types::{Address, Box, H256};

extern "C" {
	fn getSCAddress(resultOffset: *mut u8);
	fn getOwnerAddress(resultOffset: *mut u8);
	fn getShardOfAddress(address_ptr: *const u8) -> i32;
	fn isSmartContract(address_ptr: *const u8) -> i32;

	/// Currently not used.
	#[allow(dead_code)]
	fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;

	/// Currently not used.
	#[allow(dead_code)]
	fn getFunction(functionOffset: *const u8) -> i32;

	fn getCaller(resultOffset: *mut u8);

	fn getGasLeft() -> i64;
	fn getBlockTimestamp() -> i64;
	fn getBlockNonce() -> i64;
	fn getBlockRound() -> i64;
	fn getBlockEpoch() -> i64;
	fn getBlockRandomSeed(resultOffset: *mut u8);
	/// Currently not used.
	#[allow(dead_code)]
	fn getStateRootHash(resultOffset: *mut u8);
	fn getPrevBlockTimestamp() -> i64;
	fn getPrevBlockNonce() -> i64;
	fn getPrevBlockRound() -> i64;
	fn getPrevBlockEpoch() -> i64;
	fn getPrevBlockRandomSeed(resultOffset: *const u8);
	fn getOriginalTxHash(resultOffset: *const u8);

	// big int API
	fn bigIntNew(value: i64) -> i32;
	fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);

	// ESDT
	fn getCurrentESDTNFTNonce(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
	) -> i64;
	fn getESDTBalance(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
		nonce: i64,
		resultOffset: i32,
	);
}

impl ContractHookApi<ArwenBigInt, ArwenBigUint> for ArwenApiImpl {
	type Storage = Self;
	type CallValue = Self;
	type SendApi = Self;

	#[inline]
	fn get_storage_raw(&self) -> Self::Storage {
		self.clone()
	}

	#[inline]
	fn call_value(&self) -> Self::CallValue {
		self.clone()
	}

	#[inline]
	fn send(&self) -> Self::SendApi {
		self.clone()
	}

	#[inline]
	fn get_sc_address(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getSCAddress(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_owner_address(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getOwnerAddress(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_shard_of_address(&self, address: &Address) -> u32 {
		unsafe { getShardOfAddress(address.as_ref().as_ptr()) as u32 }
	}

	#[inline]
	fn is_smart_contract(&self, address: &Address) -> bool {
		unsafe { isSmartContract(address.as_ref().as_ptr()) > 0 }
	}

	#[inline]
	fn get_caller(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getCaller(res.as_mut_ptr());
			res
		}
	}

	fn get_balance(&self, address: &Address) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetExternalBalance(address.as_ref().as_ptr(), result);
			ArwenBigUint { handle: result }
		}
	}

	#[inline]
	fn get_tx_hash(&self) -> H256 {
		unsafe {
			let mut res = H256::zero();
			getOriginalTxHash(res.as_mut_ptr());
			res.into()
		}
	}

	#[inline]
	fn get_gas_left(&self) -> u64 {
		unsafe { getGasLeft() as u64 }
	}

	#[inline]
	fn get_block_timestamp(&self) -> u64 {
		unsafe { getBlockTimestamp() as u64 }
	}

	#[inline]
	fn get_block_nonce(&self) -> u64 {
		unsafe { getBlockNonce() as u64 }
	}

	#[inline]
	fn get_block_round(&self) -> u64 {
		unsafe { getBlockRound() as u64 }
	}

	#[inline]
	fn get_block_epoch(&self) -> u64 {
		unsafe { getBlockEpoch() as u64 }
	}

	#[inline]
	fn get_block_random_seed(&self) -> Box<[u8; 48]> {
		unsafe {
			let mut res = [0u8; 48];
			getBlockRandomSeed(res.as_mut_ptr());
			Box::new(res)
		}
	}

	#[inline]
	fn get_prev_block_timestamp(&self) -> u64 {
		unsafe { getPrevBlockTimestamp() as u64 }
	}

	#[inline]
	fn get_prev_block_nonce(&self) -> u64 {
		unsafe { getPrevBlockNonce() as u64 }
	}

	#[inline]
	fn get_prev_block_round(&self) -> u64 {
		unsafe { getPrevBlockRound() as u64 }
	}

	#[inline]
	fn get_prev_block_epoch(&self) -> u64 {
		unsafe { getPrevBlockEpoch() as u64 }
	}

	#[inline]
	fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
		unsafe {
			let mut res = [0u8; 48];
			getPrevBlockRandomSeed(res.as_mut_ptr());
			Box::new(res)
		}
	}

	#[inline]
	fn get_current_esdt_nft_nonce(&self, address: &Address, token: &[u8]) -> u64 {
		unsafe {
			getCurrentESDTNFTNonce(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
			) as u64
		}
	}

	#[inline]
	fn get_esdt_balance(&self, address: &Address, token: &[u8], nonce: u64) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			getESDTBalance(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				nonce as i64,
				result,
			);
			
			ArwenBigUint { handle: result }
		}
	}
}
