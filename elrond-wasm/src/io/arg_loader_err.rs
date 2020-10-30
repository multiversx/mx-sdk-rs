use crate::*;
use elrond_codec::DecodeError;
use core::marker::PhantomData;

pub fn load_arg_error<A, BigInt, BigUint>(api: &A, arg_id: ArgId, de_err: DecodeError) -> !
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    let mut decode_err_message: Vec<u8> = Vec::new();
    decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_1);
    decode_err_message.extend_from_slice(arg_id);
    decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_2);
    decode_err_message.extend_from_slice(de_err.message_bytes());
    api.signal_error(decode_err_message.as_slice())
}

pub trait DynArgErrHandler {
    fn handle_sc_error(&self, err: SCError) -> !;
}

// TODO: split ContractIOApi and maybe we won't need this struct anymore
pub struct DynEndpointErrHandler<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static 
{
    api: &'a A,
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> DynEndpointErrHandler<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static 
{
    pub fn new(api: &'a A) -> Self {
        DynEndpointErrHandler {
            api,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> DynArgErrHandler for DynEndpointErrHandler<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    fn handle_sc_error(&self, err: SCError) -> ! {
        self.api.signal_error(err.as_bytes())
    }
}
