multiversx_sc::imports!();

/// Checks that BigUint/BigInt operators work as expected.
#[multiversx_sc::module]
#[allow(clippy::redundant_clone)]
pub trait SmallIntOperators {
    #[endpoint]
    fn add_usize(&self, a: usize, b: usize) -> usize {
        a + b
    }

    #[endpoint]
    fn add_u8(&self, a: u8, b: u8) -> u8 {
        a + b
    }

    #[endpoint]
    fn add_u16(&self, a: u16, b: u16) -> u16 {
        a + b
    }

    #[endpoint]
    fn add_u32(&self, a: u32, b: u32) -> u32 {
        a + b
    }

    #[endpoint]
    fn add_u64(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}
