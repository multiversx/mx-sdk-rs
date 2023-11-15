multiversx_sc::imports!();

/// Used for testing overflow on small int types
#[multiversx_sc::module]
#[allow(clippy::redundant_clone)]
pub trait SmallIntOverflow {
    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_usize(&self) -> usize {
        usize::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_u8(&self) -> u8 {
        u8::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_u16(&self) -> u16 {
        u16::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_u32(&self) -> u32 {
        u32::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_u64(&self) -> u64 {
        u64::MAX + 1
    }
    #[endpoint]
    fn overflow_usize(&self) -> usize {
        usize::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn overflow_u8(&self) -> u8 {
        u8::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn overflow_u16(&self) -> u16 {
        u16::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn overflow_u32(&self) -> u32 {
        u32::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn overflow_u64(&self) -> u64 {
        u64::MAX.wrapping_add(1)
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_isize(&self) -> isize {
        isize::MAX + isize::MAX
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_i8(&self) -> i8 {
        i8::MAX + i8::MAX
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_i16(&self) -> i16 {
        i16::MAX + i16::MAX
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_i32(&self) -> i32 {
        i32::MAX + i32::MAX
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn no_overflow_i64(&self) -> i64 {
        i64::MAX + i64::MAX
    }

    #[endpoint]
    fn overflow_isize(&self) -> isize {
        isize::MAX.wrapping_add(isize::MAX)
    }

    #[endpoint]
    fn overflow_i8(&self) -> i8 {
        i8::MAX.wrapping_add(i8::MAX)
    }

    #[endpoint]
    fn overflow_i16(&self) -> i16 {
        i16::MAX.wrapping_add(i16::MAX)
    }

    #[endpoint]
    fn overflow_i32(&self) -> i32 {
        i32::MAX.wrapping_add(i32::MAX)
    }

    #[endpoint]
    fn overflow_i64(&self) -> i64 {
        i64::MAX.wrapping_add(i64::MAX)
    }
}
