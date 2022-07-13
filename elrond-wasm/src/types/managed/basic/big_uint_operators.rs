use crate::{
    api::{const_handles, BigIntApi, ManagedTypeApi, StaticVarApiImpl},
    types::{BigUint, ManagedType},
};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<Self> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: BigUint<M>) -> BigUint<M> {
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigUint::from_handle(self.handle.clone())
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigUint<M>> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: &BigUint<M>) -> BigUint<M> {
                let result_handle: M::BigIntHandle = M::static_var_api_impl().next_handle();
                M::managed_type_impl().$api_func(
                    result_handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigUint::from_handle(result_handle)
            }
        }

        impl<'b, M: ManagedTypeApi> $trait<&'b BigUint<M>> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: &BigUint<M>) -> BigUint<M> {
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
                BigUint::from_handle(self.handle.clone())
            }
        }

        impl<M: ManagedTypeApi> $trait<u32> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u32) -> BigUint<M> {
                let big_int_temp_1 = Self::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    big_int_temp_1,
                );
                BigUint::from_handle(self.handle.clone())
            }
        }

        impl<'a, M: ManagedTypeApi> $trait<u32> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u32) -> BigUint<M> {
                let big_int_temp_1 =
                    BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
                let result_handle: M::BigIntHandle = M::static_var_api_impl().next_handle();
                M::managed_type_impl().$api_func(
                    result_handle.clone(),
                    self.handle.clone(),
                    big_int_temp_1,
                );
                BigUint::from_handle(result_handle)
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u64) -> BigUint<M> {
                let big_int_temp_1 = Self::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    big_int_temp_1,
                );
                BigUint::from_handle(self.handle.clone())
            }
        }

        impl<'a, M: ManagedTypeApi> $trait<u64> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: u64) -> BigUint<M> {
                let big_int_temp_1 =
                    BigUint::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
                let result_handle: M::BigIntHandle = M::static_var_api_impl().next_handle();
                M::managed_type_impl().$api_func(
                    result_handle.clone(),
                    self.handle.clone(),
                    big_int_temp_1,
                );
                BigUint::from_handle(result_handle)
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
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigUint<M>> for BigUint<M> {
            #[inline]
            fn $method(&mut self, other: &BigUint<M>) {
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    other.handle.clone(),
                );
            }
        }

        impl<M: ManagedTypeApi> $trait<u32> for BigUint<M> {
            fn $method(&mut self, other: u32) {
                let big_int_temp_1 = Self::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    big_int_temp_1,
                );
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for BigUint<M> {
            fn $method(&mut self, other: u64) {
                let big_int_temp_1 = Self::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
                M::managed_type_impl().$api_func(
                    self.handle.clone(),
                    self.handle.clone(),
                    big_int_temp_1,
                );
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
                M::managed_type_impl().$api_func(self.handle.clone(), self.handle.clone(), rhs);
                self
            }
        }

        impl<'a, M: ManagedTypeApi> $shift_trait<usize> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, rhs: usize) -> BigUint<M> {
                let result_handle: M::BigIntHandle = M::static_var_api_impl().next_handle();
                M::managed_type_impl().$api_func(result_handle.clone(), self.handle.clone(), rhs);
                BigUint::from_handle(result_handle)
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
                M::managed_type_impl().$api_func(self.handle.clone(), self.handle.clone(), rhs);
            }
        }
    };
}

shift_assign_traits! {ShrAssign, shr_assign, bi_shr}
shift_assign_traits! {ShlAssign, shl_assign, bi_shl}
