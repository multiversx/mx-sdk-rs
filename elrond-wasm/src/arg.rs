use super::*;
use crate::esd_light::*;

pub fn load_single_arg<A, BigInt, BigUint, T>(api: &A, index: i32) -> T 
where
    T: Decode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    // the compiler is smart enough to evaluate this match at compile time
    match T::TYPE_INFO {
        TypeInfo::BigUint => {
            // self must be of type BigUint
            // performing a forceful cast
            let big_uint_arg = api.get_argument_big_uint(index);
            let cast_big_uint: T = unsafe { core::mem::transmute_copy(&big_uint_arg) };
            cast_big_uint
        },
        TypeInfo::I64 => {
            let arg_i64 = api.get_argument_i64(index);
            let arg_t: T = unsafe { core::mem::transmute_copy(&arg_i64) };
            arg_t
        },
        TypeInfo::I32 => {
            let arg_i64 = api.get_argument_i32(index);
            let arg_t: T = unsafe { core::mem::transmute_copy(&arg_i64) };
            arg_t
        },
        TypeInfo::I8 => {
            let arg_i64 = api.get_argument_i8(index);
            let arg_t: T = unsafe { core::mem::transmute_copy(&arg_i64) };
            arg_t
        },
        _ => {
            let arg_bytes = api.get_argument_vec(index);
            match esd_light::decode_from_byte_slice(arg_bytes.as_slice()) {
                Ok(v) => v,
                Err(de_err) => {
                    let mut decode_err_message: Vec<u8> = Vec::new();
                    decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR);
                    decode_err_message.extend_from_slice(de_err.message_bytes());
                    api.signal_error(decode_err_message.as_slice())
                }
            }
        }
    }
}

pub trait EndpointVarArgs<A, BigInt, BigUint>: Sized
where
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static 
{
    fn load(api: &A, index: &mut i32, num_args: i32) -> Self;

    fn load_multi_exact(api: &A, index: &mut i32, num_args_to_load: i32, num_args: i32) -> Self {
        let expected_end_index = *index + num_args_to_load;
        if expected_end_index > num_args {
            api.signal_error(err_msg::ARG_WRONG_NUMBER);
        }
        let result = Self::load(api, index, expected_end_index);
        if *index != expected_end_index {
            api.signal_error(err_msg::ARG_WRONG_NUMBER);
        }
        result
    }
}

impl<A, BigInt, BigUint, T> EndpointVarArgs<A, BigInt, BigUint> for T
where
    T: Decode,
    BigUint: BigUintApi + 'static,
    BigInt: BigIntApi<BigUint> + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    fn load(api: &A, index: &mut i32, num_args: i32) -> Self {
        if let TypeInfo::Unit = T::TYPE_INFO {
            // unit type returns without loading anything
            let cast_big_uint: T = unsafe { core::mem::transmute_copy(&()) };
            return cast_big_uint;
        }

        if *index >= num_args {
            api.signal_error(err_msg::ARG_WRONG_NUMBER);
        }

        let arg: T = load_single_arg(api, *index);
        *index += 1;
        arg
    }
}

pub struct VarArgs<T>(pub Vec<T>);

impl<T> From<Vec<T>> for VarArgs<T> {
    fn from(v: Vec<T>) -> Self {
        VarArgs(v)
    }
}

impl<T> VarArgs<T> {
    #[inline]
    pub fn new() -> Self {
        VarArgs(Vec::new())
    }

    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }

}

impl<A, BigInt, BigUint, T> EndpointVarArgs<A, BigInt, BigUint> for VarArgs<T>
where
    T: EndpointVarArgs<A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    fn load(api: &A, index: &mut i32, num_args: i32) -> Self {
        let mut result_vec: Vec<T> = Vec::new();
        while *index < num_args {
            let arg = T::load(api, index, num_args);
            result_vec.push(arg);
        }
        VarArgs(result_vec)
    }
}

pub enum OptionalArg<T> {
    Some(T),
    None
}

impl<A, BigInt, BigUint, T> EndpointVarArgs<A, BigInt, BigUint> for OptionalArg<T>
where
    T: EndpointVarArgs<A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    fn load(api: &A, index: &mut i32, num_args: i32) -> Self {
        if *index < num_args - 1 {
            let arg = T::load(api, index, num_args);
            OptionalArg::Some(arg)
        } else {
            OptionalArg::None
        }
    }
}

pub struct AsyncCallError {
    pub err_code: i32,
    pub err_msg: Vec<u8>,
}

pub enum AsyncCallResult<T> {
    Ok(T),
    Err(AsyncCallError)
}

impl<A, BigInt, BigUint, T> EndpointVarArgs<A, BigInt, BigUint> for AsyncCallResult<T>
where
    T: EndpointVarArgs<A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractIOApi<BigInt, BigUint> + 'static
{
    fn load(api: &A, index: &mut i32, num_args: i32) -> Self {
        if *index >= num_args {
            api.signal_error(err_msg::ARG_WRONG_NUMBER);
        }
        let err_code: i32 = load_single_arg(api, *index);
        *index += 1;
        if err_code == 0 {
            let arg = T::load(api, index, num_args);
            AsyncCallResult::Ok(arg)
        } else {
            if *index >= num_args {
                api.signal_error(err_msg::ARG_WRONG_NUMBER);
            }
            let err_msg_bytes = api.get_argument_vec(*index);
            AsyncCallResult::Err(AsyncCallError {
                err_code: err_code,
                err_msg: err_msg_bytes,
            })
        }
    }
}

macro_rules! multi_arg_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<A, BigInt, BigUint, $($name),+> EndpointVarArgs<A, BigInt, BigUint> for $mr<$($name,)+>
            where
                $($name: EndpointVarArgs<A, BigInt, BigUint>,)+
                BigInt: BigIntApi<BigUint> + 'static,
                BigUint: BigUintApi + 'static,
                A: ContractIOApi<BigInt, BigUint> + 'static
            {
                fn load(api: &A, index: &mut i32, num_args: i32) -> Self {
                    $mr((
                        $(
                            $name::load(api, index, num_args)
                        ),+
                    ))
                }
            }

            impl<$($name,)+> $mr<$($name,)+> {
                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
                }
            }
        )+
    }
}

multi_arg_impls! {
    (MultiArg2  0 T0 1 T1)
    (MultiArg3  0 T0 1 T1 2 T2)
    (MultiArg4  0 T0 1 T1 2 T2 3 T3)
    (MultiArg5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiArg6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiArg7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiArg8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiArg9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiArg10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiArg11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiArg12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiArg13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiArg14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiArg15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiArg16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

// pub trait EndpointMultiArgs<A, BigInt, BigUint>: Sized
// where
//     BigUint: BigUintApi + 'static,
//     BigInt: BigIntApi<BigUint> + 'static,
//     A: ContractIOApi<BigInt, BigUint> + 'static 
// {
//     fn load(api: &A, starting_index: i32, num_args_to_load: i32, num_args: i32) -> (Self, i32);
// }

// impl<A, BigInt, BigUint, T> EndpointMultiArgs<A, BigInt, BigUint> for VarArgs<T>
// where
//     T: EndpointVarArgs<A, BigInt, BigUint>,
//     BigInt: BigIntApi<BigUint> + 'static,
//     BigUint: BigUintApi + 'static,
//     A: ContractIOApi<BigInt, BigUint> + 'static
// {
//     fn load(api: &A, starting_index: i32, num_args_to_load: i32, num_args: i32) -> (Self, i32) {
//         let mut current_index = starting_index;
//         let mut result_vec: Vec<T> = Vec::new();
//         while current_index < num_args - 1 {
//             let (arg, new_index) = T::load(api, current_index, num_args);
//             result_vec.push(arg);
//             current_index = new_index;
//         }
//         (VarArgs(result_vec), num_args)
//     }
// }
