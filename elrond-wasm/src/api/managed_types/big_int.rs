use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use alloc::string::String;

use super::ManagedTypeApi;

pub struct BigInt<M: ManagedTypeApi> {
    handle: i32,
    api: M,
}

// BigInt sign.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

// impl<M: ManagedTypeApi> From<ArwenBigUint> for BigInt<M> {
// 	#[inline]
// 	fn from(item: ArwenBigUint) -> Self {
// 		BigInt {
// 			handle: item.handle,
// 		}
// 	}
// }

// impl<M: ManagedTypeApi> From<i64> for BigInt<M> {
//     fn from(item: i64) -> Self {
//         unsafe {
//             BigInt {
//                 handle: bigIntNew(item),
//                 api: self.api.clone(),
//             }
//         }
//     }
// }

// impl<M: ManagedTypeApi> From<i32> for BigInt<M> {
//     fn from(item: i32) -> Self {
//         unsafe {
//             BigInt {
//                 handle: bigIntNew(item.into()),
//                 api: self.api.clone(),
//             }
//         }
//     }
// }

// impl<M: ManagedTypeApi> BigInt {
//     pub fn from_i64(value: i64) -> BigInt {
//         unsafe {
//             BigInt {
//                 handle: bigIntNew(value),
//                 api: self.api.clone(),
//             }
//         }
//     }
// }

impl<M: ManagedTypeApi> Clone for BigInt<M> {
    fn clone(&self) -> Self {
        let clone_handle = self.api.new_zero();
        self.api.add(clone_handle, clone_handle, self.handle);
        BigInt {
            handle: clone_handle,
            api: self.api.clone(),
        }
    }
}

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: BigInt<M>) -> BigInt<M> {
                self.api.$api_func(self.handle, self.handle, other.handle);
                BigInt {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigInt<M>> for &'a BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: &BigInt<M>) -> BigInt<M> {
                let result = self.api.new_zero();
                self.api.$api_func(result, self.handle, other.handle);
                BigInt {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

binary_operator! {Add, add, add}
binary_operator! {Sub, sub, sub}
binary_operator! {Mul, mul, mul}
binary_operator! {Div, div, t_div}
binary_operator! {Rem, rem, t_mod}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<BigInt<M>> for BigInt<M> {
            #[inline]
            fn $method(&mut self, other: Self) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigInt<M>> for BigInt<M> {
            #[inline]
            fn $method(&mut self, other: &BigInt<M>) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }
    };
}

binary_assign_operator! {AddAssign, add_assign, add}
binary_assign_operator! {SubAssign, sub_assign, sub}
binary_assign_operator! {MulAssign, mul_assign, mul}
binary_assign_operator! {DivAssign, div_assign, t_div}
binary_assign_operator! {RemAssign, rem_assign, t_mod}

impl<M: ManagedTypeApi> PartialEq for BigInt<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let arwen_cmp = self.api.cmp(self.handle, other.handle);
        arwen_cmp == 0
    }
}

impl<M: ManagedTypeApi> Eq for BigInt<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigInt<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigInt<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let arwen_cmp = self.api.cmp(self.handle, other.handle);
        arwen_cmp.cmp(&0)
    }
}

fn arwen_cmp_i64<M: ManagedTypeApi>(bi: &BigInt<M>, other: i64) -> i32 {
    if other == 0 {
        bi.api.sign(bi.handle)
    } else {
        bi.api.cmp(bi.handle, bi.api.new(other))
    }
}

impl<M: ManagedTypeApi> PartialEq<i64> for BigInt<M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        arwen_cmp_i64(self, *other) == 0
    }
}

impl<M: ManagedTypeApi> PartialOrd<i64> for BigInt<M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        let arwen_cmp = arwen_cmp_i64(self, *other);
        Some(arwen_cmp.cmp(&0))
    }
}

impl<M: ManagedTypeApi> Neg for BigInt<M> {
    type Output = BigInt<M>;

    fn neg(self) -> Self::Output {
        let result = self.api.new_zero();
        self.api.neg(result, self.handle);
        BigInt {
            handle: result,
            api: self.api.clone(),
        }
    }
}

// use crate::elrond_codec::*;

// impl<M: ManagedTypeApi> NestedEncode for BigInt<M> {
//     const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

//     fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
//         // TODO: vector allocation can be avoided by writing directly to dest
//         self.to_signed_bytes_be().as_slice().dep_encode(dest)
//     }

//     fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
//         &self,
//         dest: &mut O,
//         c: ExitCtx,
//         exit: fn(ExitCtx, EncodeError) -> !,
//     ) {
//         self.to_signed_bytes_be()
//             .as_slice()
//             .dep_encode_or_exit(dest, c, exit);
//     }
// }

// impl<M: ManagedTypeApi> TopEncode for BigInt<M> {
//     const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

//     #[inline]
//     fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
//         output.set_big_int_handle_or_bytes(self.handle, || self.to_signed_bytes_be());
//         Ok(())
//     }

//     #[inline]
//     fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
//         &self,
//         output: O,
//         _: ExitCtx,
//         _: fn(ExitCtx, EncodeError) -> !,
//     ) {
//         output.set_big_int_handle_or_bytes(self.handle, || self.to_signed_bytes_be());
//     }
// }

// impl<M: ManagedTypeApi> NestedDecode for BigInt<M> {
// 	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

// 	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
// 		let size = usize::dep_decode(input)?;
// 		let bytes = input.read_slice(size)?;
// 		Ok(BigInt<M>::from_signed_bytes_be(bytes))
// 	}

// 	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
// 		input: &mut I,
// 		c: ExitCtx,
// 		exit: fn(ExitCtx, DecodeError) -> !,
// 	) -> Self {
// 		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
// 		let bytes = input.read_slice_or_exit(size, c, exit);
// 		BigInt<M>::from_signed_bytes_be(bytes)
// 	}
// }

// impl<M: ManagedTypeApi> TopDecode for BigInt<M> {
// 	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

// 	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
// 		// since can_use_handle is provided constantly,
// 		// the compiler is smart enough to only ever expand one of the if branches
// 		let (can_use_handle, handle) = input.try_get_big_int_handle();
// 		if can_use_handle {
// 			Ok(BigInt { handle })
// 		} else {
// 			Ok(BigInt<M>::from_signed_bytes_be(
// 				&*input.into_boxed_slice_u8(),
// 			))
// 		}
// 	}

// 	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
// 		input: I,
// 		_: ExitCtx,
// 		_: fn(ExitCtx, DecodeError) -> !,
// 	) -> Self {
// 		// since can_use_handle is provided constantly,
// 		// the compiler is smart enough to only ever expand one of the if branches
// 		let (can_use_handle, handle) = input.try_get_big_int_handle();
// 		if can_use_handle {
// 			BigInt { handle }
// 		} else {
// 			BigInt<M>::from_signed_bytes_be(&*input.into_boxed_slice_u8())
// 		}
// 	}
// }

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigInt<M> {
    fn type_name() -> String {
        String::from("BigInt")
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    // fn abs_uint(&self) -> ArwenBigUint {
    //     unsafe {
    //         let result = self.api.new_zero();
    //         bigIntAbs(result, self.handle);
    //         ArwenBigUint { handle: result }
    //     }
    // }

    pub fn sign(&self) -> Sign {
        let s = self.api.sign(self.handle);
        match s.cmp(&0) {
            Ordering::Greater => Sign::Plus,
            Ordering::Equal => Sign::NoSign,
            Ordering::Less => Sign::Minus,
        }
    }

    // fn to_signed_bytes_be(&self) -> Vec<u8> {
    // 	unsafe {
    // 		let byte_len = bigIntSignedByteLength(self.handle);
    // 		let mut vec = vec![0u8; byte_len as usize];
    // 		bigIntGetSignedBytes(self.handle, vec.as_mut_ptr());
    // 		vec
    // 	}
    // }

    // fn from_signed_bytes_be(bytes: &[u8]) -> Self {
    // 	unsafe {
    // 		let handle = self.api.new_zero();
    // 		bigIntSetSignedBytes(handle, bytes.as_ptr(), bytes.len() as i32);
    // 		BigInt { handle }
    // 	}
    // }

    pub fn to_i64(&self) -> Option<i64> {
        let is_i64_result = self.api.is_int64(self.handle);
        if is_i64_result > 0 {
            Some(self.api.get_int64(self.handle))
        } else {
            None
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let handle = self.api.new_zero();
        let exp_handle = self.api.new(exp as i64);
        self.api.pow(handle, self.handle, exp_handle);
        BigInt {
            handle,
            api: self.api.clone(),
        }
    }
}
