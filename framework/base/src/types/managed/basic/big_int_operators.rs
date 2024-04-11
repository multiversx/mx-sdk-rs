use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{
    api::{use_raw_handle, BigIntApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::{BigInt, BigUint, ManagedType, Sign},
};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<'a, M: ManagedTypeApi<'a>> $trait for BigInt<'a, M> {
            type Output = BigInt<'a, M>;

            fn $method(self, other: BigInt<'a, M>) -> BigInt<'a, M> {
                let api = M::managed_type_impl();
                api.$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigInt::from_handle(self.handle.clone())
            }
        }

        impl<'a, M: ManagedTypeApi<'a>> $trait<BigUint<'a, M>> for BigInt<'a, M> {
            type Output = BigInt<'a, M>;

            fn $method(self, other: BigUint<'a, M>) -> BigInt<'a, M> {
                self.$method(BigInt::from_biguint(Sign::Plus, other))
            }
        }

        impl<'a, M: ManagedTypeApi<'a>> $trait<BigInt<'a, M>> for BigUint<'a, M> {
            type Output = BigInt<'a, M>;

            fn $method(self, other: BigInt<'a, M>) -> BigInt<'a, M> {
                BigInt::from_biguint(Sign::Plus, self).$method(other)
            }
        }

        impl<'a, 'b, M: ManagedTypeApi<'a>> $trait<&'b BigInt<'a, M>> for &'a BigInt<'a, M> {
            type Output = BigInt<'a, M>;

            fn $method(self, other: &BigInt<'a, M>) -> BigInt<'a, M> {
                let api = M::managed_type_impl();
                let result_handle: M::BigIntHandle =
                    use_raw_handle(M::static_var_api_impl().next_handle());
                api.$api_func(
                    result_handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigInt::from_handle(result_handle)
            }
        }

        impl<'a, 'b, M: ManagedTypeApi<'a>> $trait<&'b BigUint<'a, M>> for &'a BigInt<'a, M> {
            type Output = BigInt<'a, M>;

            fn $method(self, other: &BigUint<'a, M>) -> BigInt<'a, M> {
                self.$method(&BigInt::from_handle(other.get_handle()))
            }
        }

        impl<'a, 'b, M: ManagedTypeApi<'a>> $trait<&'b BigInt<'a, M>> for &'a BigUint<'a, M> {
            type Output = BigInt<'a, M>;

            fn $method(self, other: &BigInt<'a, M>) -> BigInt<'a, M> {
                (&BigInt::from_handle(self.get_handle())).$method(other)
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
        impl<'a, M: ManagedTypeApi<'a>> $trait<BigInt<'a, M>> for BigInt<'a, M> {
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

        impl<'a, M: ManagedTypeApi<'a>> $trait<&BigInt<'a, M>> for BigInt<'a, M> {
            #[inline]
            fn $method(&mut self, other: &BigInt<'a, M>) {
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

binary_assign_operator! {AddAssign, add_assign, bi_add}
binary_assign_operator! {SubAssign, sub_assign, bi_sub}
binary_assign_operator! {MulAssign, mul_assign, bi_mul}
binary_assign_operator! {DivAssign, div_assign, bi_t_div}
binary_assign_operator! {RemAssign, rem_assign, bi_t_mod}

impl<'a, M: ManagedTypeApi<'a>> Neg for BigInt<'a, M> {
    type Output = BigInt<'a, M>;

    fn neg(self) -> Self::Output {
        let api = M::managed_type_impl();
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_neg(result_handle.clone(), self.handle);
        BigInt::from_handle(result_handle)
    }
}
