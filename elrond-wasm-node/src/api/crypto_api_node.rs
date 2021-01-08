use elrond_wasm::types::H256;
use crate::ArwenApiImpl;
use elrond_wasm::api::CryptoApi;

#[rustfmt::skip]
extern {
	fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
    fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
}

impl CryptoApi for ArwenApiImpl {
	fn sha256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}
}
