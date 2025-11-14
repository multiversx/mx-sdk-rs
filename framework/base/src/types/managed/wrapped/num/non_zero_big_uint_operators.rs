use crate::{
    api::ManagedTypeApi,
    types::{BigUint, NonZeroBigUint},
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! nz_unchecked_binary_operator {
    ($trait:ident, $method:ident, $wrap_method:ident) => {
        impl<M: ManagedTypeApi> $trait<NonZeroBigUint<M>> for NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            #[inline]
            fn $method(self, other: NonZeroBigUint<M>) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.value.$method(&other.value))
            }
        }

        impl<'b, M: ManagedTypeApi> $trait<&'b NonZeroBigUint<M>> for NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: &NonZeroBigUint<M>) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.value.$method(&other.value))
            }
        }

        impl<'b, M: ManagedTypeApi> $trait<NonZeroBigUint<M>> for &'b NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: NonZeroBigUint<M>) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.as_big_int().$method(&other.value))
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b NonZeroBigUint<M>> for &'a NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: &NonZeroBigUint<M>) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.as_big_int().$method(&other.value))
            }
        }
    };
}

nz_unchecked_binary_operator! {Add, add, wrap_big_int_unchecked}
nz_unchecked_binary_operator! {Sub, sub, wrap_big_int_assert_gt_zero}
nz_unchecked_binary_operator! {Mul, mul, wrap_big_int_unchecked}
nz_unchecked_binary_operator! {Div, div, wrap_big_int_assert_gt_zero}

macro_rules! nz_unchecked_assign_operator {
    ($trait:ident, $method:ident, $validate_method:ident) => {
        impl<M: ManagedTypeApi> $trait<NonZeroBigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: NonZeroBigUint<M>) {
                self.value.$method(other.into_big_int());
                self.$validate_method();
            }
        }

        impl<M: ManagedTypeApi> $trait<&NonZeroBigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: &NonZeroBigUint<M>) {
                self.value.$method(other.as_big_int());
                self.$validate_method();
            }
        }

        impl<M: ManagedTypeApi> $trait<BigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: BigUint<M>) {
                self.value.$method(other.into_big_int());
                self.$validate_method();
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: &BigUint<M>) {
                self.value.$method(other.as_big_int());
                self.$validate_method();
            }
        }

        impl<M: ManagedTypeApi> $trait<u32> for NonZeroBigUint<M> {
            fn $method(&mut self, other: u32) {
                unsafe {
                    self.as_big_uint_mut().$method(other);
                }
                self.$validate_method();
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for NonZeroBigUint<M> {
            fn $method(&mut self, other: u64) {
                unsafe {
                    self.as_big_uint_mut().$method(other);
                }
                self.$validate_method();
            }
        }
    };
}

nz_unchecked_assign_operator! {AddAssign, add_assign, assume_valid_after_op}
nz_unchecked_assign_operator! {SubAssign, sub_assign, validate_after_op}
nz_unchecked_assign_operator! {MulAssign, mul_assign, assume_valid_after_op}
nz_unchecked_assign_operator! {DivAssign, div_assign, validate_after_op}
