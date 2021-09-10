use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::api::ManagedTypeApi;

use super::BigInt;

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
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, other.handle);
                BigInt {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

binary_operator! {Add, add, bi_add}
binary_operator! {Sub, sub, bi_sub}
binary_operator! {Mul, mul, bi_mul}
binary_operator! {Div, div, bi_t_div}
binary_operator! {Rem, rem, bi_t_mod}

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

binary_assign_operator! {AddAssign, add_assign, bi_add}
binary_assign_operator! {SubAssign, sub_assign, bi_sub}
binary_assign_operator! {MulAssign, mul_assign, bi_mul}
binary_assign_operator! {DivAssign, div_assign, bi_t_div}
binary_assign_operator! {RemAssign, rem_assign, bi_t_mod}

impl<M: ManagedTypeApi> Neg for BigInt<M> {
    type Output = BigInt<M>;

    fn neg(self) -> Self::Output {
        let result = self.api.bi_new_zero();
        self.api.bi_neg(result, self.handle);
        BigInt {
            handle: result,
            api: self.api,
        }
    }
}
