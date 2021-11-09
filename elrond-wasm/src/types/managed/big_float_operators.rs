use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::api::ManagedTypeApi;

use super::BigFloat;

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigFloat<M> {
            type Output = BigFloat<M>;

            fn $method(self, other: BigFloat<M>) -> BigFloat<M> {
                self.api.$api_func(self.handle, self.handle, other.handle);
                BigFloat {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigFloat<M>> for &'a BigFloat<M> {
            type Output = BigFloat<M>;

            fn $method(self, other: &BigFloat<M>) -> BigFloat<M> {
                let result = self.api.bf_new_zero();
                self.api.$api_func(result, self.handle, other.handle);
                BigFloat {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

binary_operator! {Add, add, bf_add}
binary_operator! {Sub, sub, bf_sub}
binary_operator! {Mul, mul, bf_mul}
binary_operator! {Div, div, bf_div}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<BigFloat<M>> for BigFloat<M> {
            #[inline]
            fn $method(&mut self, other: Self) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigFloat<M>> for BigFloat<M> {
            #[inline]
            fn $method(&mut self, other: &BigFloat<M>) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }
    };
}

binary_assign_operator! {AddAssign, add_assign, bf_add}
binary_assign_operator! {SubAssign, sub_assign, bf_sub}
binary_assign_operator! {MulAssign, mul_assign, bf_mul}
binary_assign_operator! {DivAssign, div_assign, bf_div}

impl<M: ManagedTypeApi> Neg for BigFloat<M> {
    type Output = BigFloat<M>;

    fn neg(self) -> Self::Output {
        let result = self.api.bf_new_zero();
        self.api.bf_neg(result, self.handle);
        BigFloat {
            handle: result,
            api: self.api,
        }
    }
}
