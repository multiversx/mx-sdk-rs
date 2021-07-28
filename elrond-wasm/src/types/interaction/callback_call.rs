use crate::hex_call_data::HexCallDataSerializer;
use crate::types::ArgBuffer;

pub struct CallbackCall {
    // TODO: maybe also convert this to the more lightweight ArgBuffer at some point
    pub closure_data: HexCallDataSerializer,
}

impl CallbackCall {
    pub fn new(callback_name: &[u8]) -> Self {
        CallbackCall {
            closure_data: HexCallDataSerializer::new(callback_name),
        }
    }

    pub fn from_arg_buffer(endpoint_name: &[u8], arg_buffer: &ArgBuffer) -> Self {
        CallbackCall::from_raw(HexCallDataSerializer::from_arg_buffer(
            endpoint_name,
            arg_buffer,
        ))
    }

    pub fn from_raw(closure_data: HexCallDataSerializer) -> Self {
        CallbackCall { closure_data }
    }

    pub fn push_callback_argument_raw_bytes(&mut self, bytes: &[u8]) {
        self.closure_data.push_argument_bytes(bytes);
    }
}
