multiversx_sc::imports!();

/// Used for testing overflow on small int types
#[multiversx_sc::module]
#[allow(clippy::redundant_clone)]
pub trait SmallIntOverflow {
    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn usize(&self) -> usize {
        usize::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn u8(&self) -> u8 {
        u8::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn u16(&self) -> u16 {
        u16::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn u32(&self) -> u32 {
        u32::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn u64(&self) -> u64 {
        u64::MAX + 1
    }
    #[endpoint]
    fn usize_overflow(&self) -> usize {
        usize::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn u8_overflow(&self) -> u8 {
        u8::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn u16_overflow(&self) -> u16 {
        u16::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn u32_overflow(&self) -> u32 {
        u32::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn u64_overflow(&self) -> u64 {
        u64::MAX.wrapping_add(1)
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn isize(&self) -> isize {
        isize::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn i8(&self) -> i8 {
        i8::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn i16(&self) -> i16 {
        i16::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn i32(&self) -> i32 {
        i32::MAX + 1
    }

    #[endpoint]
    #[allow(arithmetic_overflow)]
    fn i64(&self) -> i64 {
        i64::MAX + 1
    }

    #[endpoint]
    fn isize_overflow(&self) -> isize {
        isize::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn i8_overflow(&self) -> i8 {
        i8::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn i16_overflow(&self) -> i16 {
        i16::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn i32_overflow(&self) -> i32 {
        i32::MAX.wrapping_add(1)
    }

    #[endpoint]
    fn i64_overflow(&self) -> i64 {
        i64::MAX.wrapping_add(1)
    }
}
