use crate::{
    api::{ErrorApi, ManagedTypeApi},
    hex_call_data::HexCallDataSerializer,
    types::{ManagedBuffer, ManagedType},
    ContractCallArg,
};

use super::ManagedArgBuffer;

pub struct CallbackCall<M: ManagedTypeApi> {
    callback_name: ManagedBuffer<M>,
    /// This refers to the callback args (the closure) that gets saved to storage
    arg_buffer: ManagedArgBuffer<M>,
    // TODO: maybe also convert this to the more lightweight ArgBuffer at some point
    // pub closure_data: HexCallDataSerializer,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `CallbackCall::<SA, R>::new`, here types can be inferred from the context.
pub fn new_callback_call<A>(api: A, callback_name_slice: &'static [u8]) -> CallbackCall<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    let callback_name = ManagedBuffer::new_from_bytes(api, callback_name_slice);
    CallbackCall::new(callback_name)
}

impl<M: ManagedTypeApi> CallbackCall<M> {
    pub fn new(callback_name: ManagedBuffer<M>) -> Self {
        let type_manager = callback_name.type_manager();
        let arg_buffer = ManagedArgBuffer::new_empty(type_manager);
        // CallbackCall {
        //     closure_data: HexCallDataSerializer::new(callback_name),
        // }
        CallbackCall {
            callback_name,
            arg_buffer,
        }
    }

    // pub fn from_arg_buffer(endpoint_name: &[u8], arg_buffer: &ArgBuffer) -> Self {
    //     CallbackCall::from_raw(HexCallDataSerializer::from_arg_buffer(
    //         endpoint_name,
    //         arg_buffer,
    //     ))
    // }

    // pub fn from_raw(closure_data: HexCallDataSerializer) -> Self {
    //     CallbackCall { closure_data }
    // }

    // pub fn push_callback_argument_raw_bytes(&mut self, bytes: &[u8]) {
    //     self.closure_data.push_argument_bytes(bytes);
    // }

    pub fn push_endpoint_arg<D: ContractCallArg>(&mut self, endpoint_arg: D) {
        endpoint_arg.push_dyn_arg(&mut self.arg_buffer);
    }

    pub fn serialize_hex_call_data(&self) -> HexCallDataSerializer {
        HexCallDataSerializer::from_managed_arg_buffer(&self.callback_name, &self.arg_buffer)
    }
}
