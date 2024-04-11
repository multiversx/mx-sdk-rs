use super::BigFloat;
use crate::{
    api::{use_raw_handle, BigFloatApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::managed::managed_type_trait::ManagedType,
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<'a, M: ManagedTypeApi<'a>> $trait for BigFloat<'a, M> {
            type Output = BigFloat<'a, M>;

            fn $method(self, other: BigFloat<'a, M>) -> BigFloat<'a, M> {
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigFloat::from_handle(self.handle.clone())
            }
        }

        impl<'a, 'b, M: ManagedTypeApi<'a>> $trait<&'b BigFloat<'a, M>> for &'a BigFloat<'a, M> {
            type Output = BigFloat<'a, M>;

            fn $method(self, other: &BigFloat<'a, M>) -> BigFloat<'a, M> {
                let result_handle: M::BigFloatHandle =
                    use_raw_handle(M::static_var_api_impl().next_handle());
                M::managed_type_impl().$api_func(
                    result_handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigFloat::from_handle(result_handle)
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
        impl<'a, M: ManagedTypeApi<'a>> $trait<BigFloat<'a, M>> for BigFloat<'a, M> {
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

        impl<'a, M: ManagedTypeApi<'a>> $trait<&BigFloat<'a, M>> for BigFloat<'a, M> {
            #[inline]
            fn $method(&mut self, other: &BigFloat<'a, M>) {
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

impl<'a, M: ManagedTypeApi<'a>> Neg for BigFloat<'a, M> {
    type Output = BigFloat<'a, M>;

    fn neg(self) -> Self::Output {
        let result_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_neg(result_handle.clone(), self.handle);
        BigFloat::from_handle(result_handle)
    }
}
