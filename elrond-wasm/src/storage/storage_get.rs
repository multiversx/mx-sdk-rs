use crate::api::{ErrorApi, StorageReadApi};
use crate::err_msg;
use crate::types::BoxedBytes;
use alloc::boxed::Box;
use elrond_codec::*;

struct StorageGetInput<'k, SRA>
where
	SRA: StorageReadApi + ErrorApi + 'static,
{
	api: SRA,
	key: &'k [u8],
}

impl<'k, SRA> StorageGetInput<'k, SRA>
where
	SRA: StorageReadApi + ErrorApi + 'static,
{
	#[inline]
	fn new(api: SRA, key: &'k [u8]) -> Self {
		StorageGetInput { api, key }
	}
}

impl<'k, SRA> TopDecodeInput for StorageGetInput<'k, SRA>
where
	SRA: StorageReadApi + ErrorApi + 'static,
{
	fn byte_len(&self) -> usize {
		self.api.storage_load_len(self.key)
	}

	fn into_boxed_slice_u8(self) -> Box<[u8]> {
		self.api.storage_load_boxed_bytes(self.key).into_box()
	}

	fn into_u64(self) -> u64 {
		self.api.storage_load_u64(self.key)
	}

	fn into_i64(self) -> i64 {
		self.api.storage_load_i64(self.key)
	}

	fn try_get_big_uint_handle(&self) -> (bool, i32) {
		(true, self.api.storage_load_big_uint_raw(self.key))
	}

	// TODO: there is currently no API hook for storage of signed big ints
}

pub fn storage_get<SRA, T>(api: SRA, key: &[u8]) -> T
where
	T: TopDecode,
	SRA: StorageReadApi + ErrorApi + Clone + 'static,
{
	T::top_decode_or_exit(
		StorageGetInput::new(api.clone(), key),
		api,
		storage_get_exit,
	)
}

#[inline(always)]
fn storage_get_exit<SRA>(api: SRA, de_err: DecodeError) -> !
where
	SRA: StorageReadApi + ErrorApi + 'static,
{
	let decode_err_message =
		BoxedBytes::from_concat(&[err_msg::STORAGE_DECODE_ERROR, de_err.message_bytes()][..]);
	api.signal_error(decode_err_message.as_slice())
}
