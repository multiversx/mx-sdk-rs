use crate::*;
use elrond_codec::TopDecodeInput;
use core::marker::PhantomData;

/// Adapter from the API to the TopDecodeInput trait.
/// Allows objects to be deserialized directly from the API as arguments.
/// 
/// Of course the implementation provides shortcut deserialization computation paths directly from API:
/// get_u64, get_i64, ...
/// 
/// This is a performance-critical struct.
/// Since the wasm ContractIOApi is zero-size,
/// it means that this structures translates to a single glorified i32 in wasm.
pub struct ArgDecodeInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    api: &'a A,
    arg_index: i32,
    boxed_value: Box<[u8]>, // TODO: remove
    _phantom1: PhantomData<BigInt>,
    _phantom2: PhantomData<BigUint>,
}

impl<'a, A, BigInt, BigUint> ArgDecodeInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    #[inline]
    pub fn new(api: &'a A, arg_index: i32) -> Self {
        ArgDecodeInput {
            api,
            arg_index,
            boxed_value: Box::new([]),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, A, BigInt, BigUint> TopDecodeInput for ArgDecodeInput<'a, A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'a 
{
    fn byte_len(&self) -> usize {
        self.api.get_argument_len(self.arg_index)
    }

    fn get_slice_u8(&mut self) -> &[u8] {
        self.boxed_value = self.api.get_argument_boxed_slice_u8(self.arg_index);
        &*self.boxed_value
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.get_argument_boxed_slice_u8(self.arg_index)
    }

    fn get_u64(&mut self) -> u64 {
        self.api.get_argument_u64(self.arg_index)
    }

    fn get_i64(&mut self) -> i64 {
        self.api.get_argument_i64(self.arg_index)
    }
}
