use super::BigFloat;
use crate::{
    api::{BigFloatApiImpl, ManagedTypeApi},
    types::managed::managed_type_trait::ManagedType,
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigFloat<M> {
            type Output = BigFloat<M>;

            fn $method(self, other: BigFloat<M>) -> BigFloat<M> {
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                self
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigFloat<M>> for &'a BigFloat<M> {
            type Output = BigFloat<M>;

            fn $method(self, other: &BigFloat<M>) -> BigFloat<M> {
                unsafe {
                    let result = BigFloat::new_uninit();
                    M::managed_type_impl().$api_func(
                        result.get_handle(),
                        self.handle.clone(),
                        other.handle.clone(),
                    );
                    result
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
                let api = M::managed_type_impl();
                api.$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigFloat<M>> for BigFloat<M> {
            #[inline]
            fn $method(&mut self, other: &BigFloat<M>) {
                let api = M::managed_type_impl();
                api.$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
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
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_neg(result.get_handle(), self.handle.clone());
            result
        }
    }
}
