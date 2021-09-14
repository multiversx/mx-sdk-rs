use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::api::ManagedTypeApi;

use super::BigUint;

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<Self> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: BigUint<M>) -> BigUint<M> {
                self.api.$api_func(self.handle, self.handle, other.handle);
                BigUint {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigUint<M>> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: &BigUint<M>) -> BigUint<M> {
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, other.handle);
                BigUint {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }

        impl<'b, M: ManagedTypeApi> $trait<&'b BigUint<M>> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: &BigUint<M>) -> BigUint<M> {
                self.api.$api_func(self.handle, self.handle, other.handle);
                BigUint {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<M: ManagedTypeApi> $trait<u32> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u32) -> BigUint<M> {
                let other_handle = self.api.bi_new(other as i64);
                self.api.$api_func(self.handle, self.handle, other_handle);
                BigUint {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, M: ManagedTypeApi> $trait<u32> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u32) -> BigUint<M> {
                let other_handle = self.api.bi_new(other as i64);
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, other_handle);
                BigUint {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u64) -> BigUint<M> {
                let other_handle = self.api.bi_new(other as i64);
                self.api.$api_func(self.handle, self.handle, other_handle);
                BigUint {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, M: ManagedTypeApi> $trait<u64> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u64) -> BigUint<M> {
                let other_handle = self.api.bi_new(other as i64);
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, other_handle);
                BigUint {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

binary_operator! {Add, add, bi_add}
binary_operator! {Sub, sub, bi_sub_unsigned}
binary_operator! {Mul, mul, bi_mul}
binary_operator! {Div, div, bi_t_div}
binary_operator! {Rem, rem, bi_t_mod}
binary_operator! {BitAnd, bitand, bi_and}
binary_operator! {BitOr,  bitor,  bi_or}
binary_operator! {BitXor, bitxor, bi_xor}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<BigUint<M>> for BigUint<M> {
            #[inline]
            fn $method(&mut self, other: Self) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigUint<M>> for BigUint<M> {
            #[inline]
            fn $method(&mut self, other: &BigUint<M>) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<u32> for BigUint<M> {
            fn $method(&mut self, other: u32) {
                let other_handle = self.api.bi_new(other as i64);
                self.api.$api_func(self.handle, self.handle, other_handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for BigUint<M> {
            fn $method(&mut self, other: u64) {
                let other_handle = self.api.bi_new(other as i64);
                self.api.$api_func(self.handle, self.handle, other_handle);
            }
        }
    };
}

binary_assign_operator! {AddAssign, add_assign, bi_add}
binary_assign_operator! {SubAssign, sub_assign, bi_sub_unsigned}
binary_assign_operator! {MulAssign, mul_assign, bi_mul}
binary_assign_operator! {DivAssign, div_assign, bi_t_div}
binary_assign_operator! {RemAssign, rem_assign, bi_t_mod}
binary_assign_operator! {BitAndAssign, bitand_assign, bi_and}
binary_assign_operator! {BitOrAssign,  bitor_assign,  bi_or}
binary_assign_operator! {BitXorAssign, bitxor_assign, bi_xor}

macro_rules! shift_traits {
    ($shift_trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $shift_trait<usize> for BigUint<M> {
            type Output = BigUint<M>;

            #[inline]
            fn $method(self, rhs: usize) -> BigUint<M> {
                self.api.$api_func(self.handle, self.handle, rhs);
                self
            }
        }

        impl<'a, M: ManagedTypeApi> $shift_trait<usize> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, rhs: usize) -> BigUint<M> {
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, rhs);
                BigUint {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

shift_traits! {Shr, shr, bi_shr}
shift_traits! {Shl, shl, bi_shl}

macro_rules! shift_assign_traits {
    ($shift_assign_trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $shift_assign_trait<usize> for BigUint<M> {
            #[inline]
            fn $method(&mut self, rhs: usize) {
                self.api.$api_func(self.handle, self.handle, rhs);
            }
        }
    };
}

shift_assign_traits! {ShrAssign, shr_assign, bi_shr}
shift_assign_traits! {ShlAssign, shl_assign, bi_shl}
