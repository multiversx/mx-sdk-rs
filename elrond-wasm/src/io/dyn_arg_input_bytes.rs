use crate::*;
use crate::{api::ErrorApi, types::BoxedBytes};

pub struct BytesArgLoader<'a, SE>
where
	SE: ErrorApi,
{
	bytes: &'a [BoxedBytes],
	signal_error: SE,
}

impl<'a, SE> BytesArgLoader<'a, SE>
where
	SE: ErrorApi,
{
	pub fn new(bytes: &'a [BoxedBytes], signal_error: SE) -> Self {
		BytesArgLoader {
			bytes,
			signal_error,
		}
	}
}

impl<'a, SE> ErrorApi for BytesArgLoader<'a, SE>
where
	SE: ErrorApi,
{
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		self.signal_error.signal_error(message)
	}
}

impl<'a, SE> DynArgInput<&'a [u8]> for BytesArgLoader<'a, SE>
where
	SE: ErrorApi,
{
	#[inline]
	fn has_next(&self) -> bool {
		!self.bytes.is_empty()
	}

	fn next_arg_input(&mut self) -> &'a [u8] {
		if self.bytes.is_empty() {
			self.signal_error(err_msg::ARG_WRONG_NUMBER);
		}
		let result = self.bytes[0].as_slice();
		self.bytes = &self.bytes[1..];
		result
	}
}
