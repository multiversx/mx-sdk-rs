use crate::*;
use crate::esd_light::*;


pub trait DynArgLoader<T>: Sized {
    fn has_next(&self) -> bool;

    fn next_arg(&mut self) -> Result<Option<T>, SCError>;
}

pub trait ArgType<D>: Sized {
    fn load(loader: &mut D) -> Result<Self, SCError>;

    #[inline]
    fn load_exact(_loader: &mut D, _num: usize) -> Result<Self, SCError> {
        Err(SCError::Static(&b"multi arg not supported"[..]))
    }
}

#[inline]
pub fn load_dyn_arg<T, D, E>(loader: &mut D, err_handler: &E) -> T
where
    T: ArgType<D> + 'static,
    E: DynArgErrHandler,
{
    match T::load(loader) {
        Ok(arg) => arg,
        Err(sc_err) => err_handler.handle_sc_error(sc_err),
    }
}

#[inline]
pub fn load_dyn_multi_arg<T, D, E>(loader: &mut D, err_handler: &E, num: usize) -> T
where
    T: ArgType<D> + 'static,
    E: DynArgErrHandler,
{
    match T::load_exact(loader, num) {
        Ok(arg) => arg,
        Err(sc_err) => err_handler.handle_sc_error(sc_err),
    }
}

impl<T, D> ArgType<D> for T
where
    T: Decode,
    D: DynArgLoader<T> + 'static,
{
    fn load(loader: &mut D) -> Result<Self, SCError> {
        if let TypeInfo::Unit = T::TYPE_INFO {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return Ok(cast_unit);
        }

        match loader.next_arg() {
            Ok(Some(arg)) => Ok(arg),
            Ok(None) => Err(SCError::Static(err_msg::ARG_WRONG_NUMBER)),
            Err(sc_err) => Err(sc_err),
        }
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

impl<T, D> ArgType<D> for VarArgs<T>
where
    T: ArgType<D>,
    D: DynArgLoader<T> + 'static,
{
    fn load(loader: &mut D) -> Result<Self, SCError> {
        let mut result_vec: Vec<T> = Vec::new();
        loop {
            match loader.next_arg() {
                Ok(Some(arg)) => {
                    result_vec.push(arg);
                },
                Ok(None) => {
                    return Ok(VarArgs(result_vec));
                }
                Err(sc_err) => {
                    return Err(sc_err);
                }
            }
        }
    }

    fn load_exact(loader: &mut D, num: usize) -> Result<Self, SCError> {
        let mut result_vec: Vec<T> = Vec::new();
        let mut i = 0usize;
        loop {
            if i == num {
                return Ok(VarArgs(result_vec));
            }
            i += 1;
            match loader.next_arg() {
                Ok(Some(arg)) => {
                    result_vec.push(arg);
                },
                Ok(None) => {
                    if i < num {
                        return Err(SCError::Static(err_msg::ARG_WRONG_NUMBER));
                    }
                    return Ok(VarArgs(result_vec));
                }
                Err(sc_err) => {
                    return Err(sc_err);
                }
            }
        }
    }
}

pub enum OptionalArg<T> {
    Some(T),
    None
}

impl<T> From<Option<T>> for OptionalArg<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(arg) => OptionalArg::Some(arg),
            None => OptionalArg::None
        }
    }
}

impl<T> OptionalArg<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OptionalArg::Some(arg) => Some(arg),
            OptionalArg::None => None
        }
    }
}

impl<T, D> ArgType<D> for OptionalArg<T>
where
    T: ArgType<D>,
    D: DynArgLoader<T> + 'static,
{
    fn load(loader: &mut D) -> Result<Self, SCError> {
        match loader.next_arg() {
            Ok(opt) => Ok(opt.into()),
            Err(sc_err) => Err(sc_err),
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

impl<T, D> ArgType<D> for AsyncCallResult<T>
where
    T: ArgType<D>,
    D: DynArgLoader<T> + DynArgLoader<i32> + DynArgLoader<Vec<u8>> + 'static,
{
    fn load(loader: &mut D) -> Result<Self, SCError> {
        let err_code: i32 = match loader.next_arg() {
            Ok(Some(arg)) => arg,
            Ok(None) => {
                return Err(SCError::Static(err_msg::ARG_WRONG_NUMBER));
            },
            Err(sc_err) => {
                return Err(sc_err);
            }
        };

        if err_code == 0 {
            let arg = T::load(loader)?;
            Ok(AsyncCallResult::Ok(arg))
        } else {
            let err_msg_bytes = Vec::<u8>::load(loader)?;
            Ok(AsyncCallResult::Err(AsyncCallError {
                err_code: err_code,
                err_msg: err_msg_bytes,
            }))
        }
    }
}

macro_rules! multi_arg_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<$($name,)+ D> ArgType<D> for $mr<$($name,)+>
            where
                $($name: ArgType<D>,)+
                D: $(DynArgLoader<$name> + )+ 'static
            {
                fn load(loader: &mut D) -> Result<Self, SCError> {
                    Ok($mr((
                        $(
                            $name::load(loader)?
                        ),+
                    )))
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
