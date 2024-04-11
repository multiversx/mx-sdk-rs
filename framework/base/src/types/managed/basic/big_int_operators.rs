use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{
    api::{use_raw_handle, BigIntApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::{BigInt, BigUint, ManagedType, Sign},
};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: BigInt<M>) -> BigInt<M> {
                let api = M::managed_type_impl();
                api.$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigInt::from_handle(self.handle.clone())
            }
        }

        impl<M: ManagedTypeApi> $trait<BigUint<M>> for BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: BigUint<M>) -> BigInt<M> {
                self.$method(BigInt::from_biguint(Sign::Plus, other))
            }
        }

        impl<M: ManagedTypeApi> $trait<BigInt<M>> for BigUint<M> {
            type Output = BigInt<M>;

            fn $method(self, other: BigInt<M>) -> BigInt<M> {
                BigInt::from_biguint(Sign::Plus, self).$method(other)
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigInt<M>> for &'a BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: &BigInt<M>) -> BigInt<M> {
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

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigUint<M>> for &'a BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: &BigUint<M>) -> BigInt<M> {
                self.$method(&BigInt::from_handle(other.get_handle()))
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigInt<M>> for &'a BigUint<M> {
            type Output = BigInt<M>;

            fn $method(self, other: &BigInt<M>) -> BigInt<M> {
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
        impl<M: ManagedTypeApi> $trait<BigInt<M>> for BigInt<M> {
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

        impl<M: ManagedTypeApi> $trait<&BigInt<M>> for BigInt<M> {
            #[inline]
            fn $method(&mut self, other: &BigInt<M>) {
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

impl<M: ManagedTypeApi> Neg for BigInt<M> {
    type Output = BigInt<M>;

    fn neg(self) -> Self::Output {
        let api = M::managed_type_impl();
        let result_handle: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bi_neg(result_handle.clone(), self.handle);
        BigInt::from_handle(result_handle)
    }
}
