use crate::api::{EndpointArgumentApi, ErrorApi};
use crate::err_msg;
use crate::{ArgDecodeInput, DynArgInput};

pub struct EndpointDynArgLoader<AA>
where
	AA: EndpointArgumentApi + 'static,
{
	api: AA,
	current_index: i32,
	num_arguments: i32,
}

impl<AA> EndpointDynArgLoader<AA>
where
	AA: EndpointArgumentApi + 'static,
{
	pub fn new(api: AA) -> Self {
		let num_arguments = api.get_num_arguments();
		EndpointDynArgLoader {
			api,
			current_index: 0,
			num_arguments,
		}
	}
}

impl<AA> ErrorApi for EndpointDynArgLoader<AA>
where
	AA: EndpointArgumentApi + ErrorApi + 'static,
{
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		self.api.signal_error(message)
	}
}

impl<AA> DynArgInput<ArgDecodeInput<AA>> for EndpointDynArgLoader<AA>
where
	AA: EndpointArgumentApi + Clone + 'static,
{
	fn has_next(&self) -> bool {
		self.current_index < self.num_arguments
	}

	fn next_arg_input(&mut self) -> ArgDecodeInput<AA> {
		if self.current_index >= self.num_arguments {
			self.signal_error(err_msg::ARG_WRONG_NUMBER)
		} else {
			let arg_input = ArgDecodeInput::new(self.api.clone(), self.current_index);
			self.current_index += 1;
			arg_input
		}
	}
}
