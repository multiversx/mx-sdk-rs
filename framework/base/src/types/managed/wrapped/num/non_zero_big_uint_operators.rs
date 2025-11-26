use crate::{
    api::ManagedTypeApi,
    types::{BigUint, NonZeroBigUint},
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

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

nz_binary_operator! {Add, add, wrap_big_int_unchecked}
nz_binary_operator! {Sub, sub, wrap_big_int_assert_gt_zero}
nz_binary_operator! {Mul, mul, wrap_big_int_unchecked}
nz_binary_operator! {Div, div, wrap_big_int_assert_gt_zero}
nz_binary_operator! {Rem, rem, wrap_big_int_assert_gt_zero}

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
