use crate::{
    api::{ManagedTypeApi, quick_signal_error},
    err_msg,
    types::{BigInt, BigUint, NonZeroBigUint, Sign},
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

impl<M: ManagedTypeApi> NonZeroBigUint<M> {
    /// Checks that value respects invariants. Used after some operator calls.
    fn validate_after_op(&self) {
        match self.value.sign() {
            Sign::Minus => quick_signal_error::<M>(err_msg::BIG_UINT_SUB_NEGATIVE),
            Sign::NoSign => quick_signal_error::<M>(err_msg::ZERO_VALUE_NOT_ALLOWED),
            Sign::Plus => {}
        }
    }

    fn wrap_big_int_assert_gt_zero(value: BigInt<M>) -> Self {
        let result = Self::wrap_big_int_unchecked(value);
        result.validate_after_op();
        result
    }

    /// Used in some operator definitions. Using it directly could violate invariant.
    unsafe fn as_big_uint_mut(&mut self) -> &mut BigUint<M> {
        unsafe { core::mem::transmute(self) }
    }
}

macro_rules! nz_binary_operator {
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

nz_binary_operator! {Add, add, wrap_big_int_unchecked} // non zero + non zero = guaranteed non zero
nz_binary_operator! {Sub, sub, wrap_big_int_assert_gt_zero} // non zero - non zero = needs validation
nz_binary_operator! {Mul, mul, wrap_big_int_unchecked} // non zero * non zero = guaranteed non zero
nz_binary_operator! {Div, div, wrap_big_int_assert_gt_zero} // non zero / non zero = needs validation
nz_binary_operator! {Rem, rem, wrap_big_int_assert_gt_zero} // non zero % non zero = needs validation

macro_rules! nz_binary_operator_small_int {
    ($trait:ident, $method:ident, $wrap_method:ident) => {
        impl<M: ManagedTypeApi> $trait<u32> for NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: u32) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.into_big_uint().$method(other).into_big_int())
            }
        }

        impl<'a, M: ManagedTypeApi> $trait<u32> for &'a NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: u32) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.as_big_uint().$method(other).into_big_int())
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: u64) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.into_big_uint().$method(other).into_big_int())
            }
        }

        impl<'a, M: ManagedTypeApi> $trait<u64> for &'a NonZeroBigUint<M> {
            type Output = NonZeroBigUint<M>;

            fn $method(self, other: u64) -> NonZeroBigUint<M> {
                NonZeroBigUint::$wrap_method(self.as_big_uint().$method(other).into_big_int())
            }
        }
    };
}

nz_binary_operator_small_int! {Add, add, wrap_big_int_unchecked} // non zero + unsigned = guaranteed non zero
nz_binary_operator_small_int! {Sub, sub, wrap_big_int_assert_gt_zero} // non zero - unsigned = needs validation
nz_binary_operator_small_int! {Mul, mul, wrap_big_int_assert_gt_zero} // non zero * unsigned = needs validation
nz_binary_operator_small_int! {Div, div, wrap_big_int_assert_gt_zero} // non zero / unsigned = needs validation
nz_binary_operator_small_int! {Rem, rem, wrap_big_int_assert_gt_zero} // non zero % unsigned = needs validation

// assignment operators

// AddAssign

impl<M: ManagedTypeApi> AddAssign<NonZeroBigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn add_assign(&mut self, other: NonZeroBigUint<M>) {
        self.value.add_assign(other.into_big_int());
        // no need to validate, NonZeroBigUint + NonZeroBigUint is always non-zero
    }
}

impl<M: ManagedTypeApi> AddAssign<&NonZeroBigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn add_assign(&mut self, other: &NonZeroBigUint<M>) {
        self.value.add_assign(other.as_big_int());
        // no need to validate, NonZeroBigUint + NonZeroBigUint is always non-zero
    }
}

impl<M: ManagedTypeApi> AddAssign<BigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn add_assign(&mut self, other: BigUint<M>) {
        self.value.add_assign(other.into_big_int());
        // no need to validate, NonZeroBigUint + BigUint is always non-zero
    }
}

impl<M: ManagedTypeApi> AddAssign<&BigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn add_assign(&mut self, other: &BigUint<M>) {
        self.value.add_assign(other.as_big_int());
        // no need to validate, NonZeroBigUint + BigUint is always non-zero
    }
}

impl<M: ManagedTypeApi> AddAssign<u32> for NonZeroBigUint<M> {
    fn add_assign(&mut self, other: u32) {
        unsafe {
            self.as_big_uint_mut().add_assign(other);
        }
        // no need to validate, NonZeroBigUint + u32 is always non-zero
    }
}

impl<M: ManagedTypeApi> AddAssign<u64> for NonZeroBigUint<M> {
    fn add_assign(&mut self, other: u64) {
        unsafe {
            self.as_big_uint_mut().add_assign(other);
        }
        // no need to validate, NonZeroBigUint + u64 is always non-zero
    }
}

// MulAssign

impl<M: ManagedTypeApi> MulAssign<NonZeroBigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn mul_assign(&mut self, other: NonZeroBigUint<M>) {
        self.value.mul_assign(other.into_big_int());
        // no need to validate, NonZeroBigUint * NonZeroBigUint is always non-zero
    }
}
impl<M: ManagedTypeApi> MulAssign<&NonZeroBigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn mul_assign(&mut self, other: &NonZeroBigUint<M>) {
        self.value.mul_assign(other.as_big_int());
        // no need to validate, NonZeroBigUint * NonZeroBigUint is always non-zero
    }
}

impl<M: ManagedTypeApi> MulAssign<BigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn mul_assign(&mut self, other: BigUint<M>) {
        self.value.mul_assign(other.into_big_int());
        self.validate_after_op(); // validation needed, as BigUint can be zero
    }
}

impl<M: ManagedTypeApi> MulAssign<&BigUint<M>> for NonZeroBigUint<M> {
    #[inline]
    fn mul_assign(&mut self, other: &BigUint<M>) {
        self.value.mul_assign(other.as_big_int());
        self.validate_after_op(); // validation needed, as BigUint can be zero
    }
}

impl<M: ManagedTypeApi> MulAssign<u32> for NonZeroBigUint<M> {
    fn mul_assign(&mut self, other: u32) {
        unsafe {
            self.as_big_uint_mut().mul_assign(other);
        }
        self.validate_after_op(); // validation needed, as u32 can be zero
    }
}
impl<M: ManagedTypeApi> MulAssign<u64> for NonZeroBigUint<M> {
    fn mul_assign(&mut self, other: u64) {
        unsafe {
            self.as_big_uint_mut().mul_assign(other);
        }
        self.validate_after_op(); // validation needed, as u64 can be zero
    }
}

macro_rules! nz_checked_assign_operator {
    ($trait:ident, $method:ident) => {
        impl<M: ManagedTypeApi> $trait<NonZeroBigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: NonZeroBigUint<M>) {
                self.value.$method(other.into_big_int());
                self.validate_after_op();
            }
        }

        impl<M: ManagedTypeApi> $trait<&NonZeroBigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: &NonZeroBigUint<M>) {
                self.value.$method(other.as_big_int());
                self.validate_after_op();
            }
        }

        impl<M: ManagedTypeApi> $trait<BigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: BigUint<M>) {
                self.value.$method(other.into_big_int());
                self.validate_after_op();
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigUint<M>> for NonZeroBigUint<M> {
            #[inline]
            fn $method(&mut self, other: &BigUint<M>) {
                self.value.$method(other.as_big_int());
                self.validate_after_op();
            }
        }

        impl<M: ManagedTypeApi> $trait<u32> for NonZeroBigUint<M> {
            fn $method(&mut self, other: u32) {
                unsafe {
                    self.as_big_uint_mut().$method(other);
                }
                self.validate_after_op();
            }
        }

        impl<M: ManagedTypeApi> $trait<u64> for NonZeroBigUint<M> {
            fn $method(&mut self, other: u64) {
                unsafe {
                    self.as_big_uint_mut().$method(other);
                }
                self.validate_after_op();
            }
        }
    };
}

nz_checked_assign_operator! {SubAssign, sub_assign}
nz_checked_assign_operator! {DivAssign, div_assign}
nz_checked_assign_operator! {RemAssign, rem_assign}
