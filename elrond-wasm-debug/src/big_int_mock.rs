

use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use core::ops::Mul;
use core::ops::MulAssign;

use alloc::vec::Vec;

use num_bigint::BigInt;
use core::cmp::Ordering;

pub struct RustBigInt(num_bigint::BigInt);

impl From<i64> for RustBigInt {
    fn from(item: i64) -> Self {
        RustBigInt(item.into())
    }
}

impl From<i32> for RustBigInt {
    fn from(item: i32) -> Self {
        RustBigInt(item.into())
    }
}

impl From<BigInt> for RustBigInt {
    fn from(item: BigInt) -> Self {
        RustBigInt(item)
    }
}

impl Clone for RustBigInt {
    fn clone(&self) -> Self {
        RustBigInt(self.0.clone())
    }
}

impl Add for RustBigInt {
    type Output = RustBigInt;

    fn add(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 + other.0)
    }
}

impl AddAssign<RustBigInt> for RustBigInt {
    fn add_assign(&mut self, other: Self) {
        BigInt::add_assign(&mut self.0, other.0)
    }
}

impl AddAssign<&RustBigInt> for RustBigInt {
    fn add_assign(&mut self, other: &RustBigInt) {
        BigInt::add_assign(&mut self.0, &other.0)
    }
}

impl Sub for RustBigInt {
    type Output = RustBigInt;

    fn sub(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 - other.0)
    }
}

impl SubAssign<RustBigInt> for RustBigInt {
    fn sub_assign(&mut self, other: Self) {
        BigInt::sub_assign(&mut self.0, other.0)
    }
}

impl SubAssign<&RustBigInt> for RustBigInt {
    fn sub_assign(&mut self, other: &RustBigInt) {
        BigInt::sub_assign(&mut self.0, &other.0)
    }
}

impl Mul for RustBigInt {
    type Output = RustBigInt;

    fn mul(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 * other.0)
    }
}

impl MulAssign for RustBigInt {
    fn mul_assign(&mut self, other: Self) {
        BigInt::mul_assign(&mut self.0, other.0)
    }
}

impl PartialEq for RustBigInt {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl Eq for RustBigInt{}

impl Ord for RustBigInt {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl PartialOrd for RustBigInt {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl elrond_wasm::BigIntApi for RustBigInt {
    fn byte_length(&self) -> i32 {
        panic!("byte_length not yet implemented")
    }

    fn copy_to_slice(&self, _slice: &mut [u8]) -> i32 {
        panic!("copy_to_slice not yet implemented")
    }

    fn get_bytes_big_endian(&self) -> Vec<u8> {
        let (_, be) = self.0.to_bytes_be();
        be
    }

    fn get_bytes_big_endian_pad_right(&self, nr_bytes: usize) -> Vec<u8> {
        let (_, bytes_be) = self.0.to_bytes_be();
        if bytes_be.len() > nr_bytes {
            panic!("Number doesn't fit requested bytes");
        } else if bytes_be.len() == nr_bytes {
            bytes_be
        } else {
            let mut res = vec![0u8; nr_bytes];
            let offset = nr_bytes - bytes_be.len();
            for i in 0..bytes_be.len()-1 {
                res[offset+i] = bytes_be[i];
            }
            res
        }
    }

    fn phantom() -> Self {
        RustBigInt::from(0)
    }
}

impl RustBigInt {
    pub fn to_signed_bytes_be(&self) -> Vec<u8>{
        self.0.to_signed_bytes_be()
    }
}

pub struct RustBigUint(num_bigint::BigInt);

impl From<RustBigInt> for RustBigUint {
    fn from(item: RustBigInt) -> Self {
        RustBigUint(item.0)
    }
}

impl elrond_wasm::BigUintApi<RustBigInt> for RustBigUint {
    fn into_signed(self) -> RustBigInt {
        RustBigInt(self.0)
    }

    fn phantom() -> Self {
        RustBigUint::from(RustBigInt::from(0))
    }
}
