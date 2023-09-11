#![allow(clippy::bad_bit_mask)]

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct VMCodeMetadata: u16 {
        const DEFAULT = 0;
        const UPGRADEABLE = 0b0000_0001_0000_0000; // LSB of first byte
        const READABLE = 0b0000_0100_0000_0000; // 3rd LSB of first byte
        const PAYABLE = 0b0000_0000_0000_0010; // 2nd LSB of second byte
        const PAYABLE_BY_SC = 0b0000_0000_0000_0100; // 3rd LSB of second byte
    }
}

impl VMCodeMetadata {
    pub fn is_upgradeable(&self) -> bool {
        *self & VMCodeMetadata::UPGRADEABLE != VMCodeMetadata::DEFAULT
    }

    pub fn is_payable(&self) -> bool {
        *self & VMCodeMetadata::PAYABLE != VMCodeMetadata::DEFAULT
    }

    pub fn is_payable_by_sc(&self) -> bool {
        *self & VMCodeMetadata::PAYABLE_BY_SC != VMCodeMetadata::DEFAULT
    }

    pub fn is_readable(&self) -> bool {
        *self & VMCodeMetadata::READABLE != VMCodeMetadata::DEFAULT
    }

    pub fn to_byte_array(&self) -> [u8; 2] {
        self.bits().to_be_bytes()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.to_byte_array().to_vec()
    }
}

impl From<[u8; 2]> for VMCodeMetadata {
    #[inline]
    fn from(arr: [u8; 2]) -> Self {
        VMCodeMetadata::from(u16::from_be_bytes(arr))
    }
}

impl From<u16> for VMCodeMetadata {
    #[inline]
    fn from(value: u16) -> Self {
        VMCodeMetadata::from_bits_truncate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert!(!VMCodeMetadata::DEFAULT.is_upgradeable());
        assert!(!VMCodeMetadata::DEFAULT.is_payable());
        assert!(!VMCodeMetadata::DEFAULT.is_readable());
    }

    #[test]
    fn test_all() {
        let all = VMCodeMetadata::UPGRADEABLE
            | VMCodeMetadata::PAYABLE
            | VMCodeMetadata::PAYABLE_BY_SC
            | VMCodeMetadata::READABLE;
        assert!(all.is_upgradeable());
        assert!(all.is_payable());
        assert!(all.is_payable_by_sc());
        assert!(all.is_readable());

        assert_eq!(all.bits(), 0x0506);

        assert_eq!(VMCodeMetadata::from_bits_truncate(0xffff), all);
    }

    #[test]
    fn test_each() {
        assert!(VMCodeMetadata::UPGRADEABLE.is_upgradeable());
        assert!(!VMCodeMetadata::PAYABLE.is_upgradeable());
        assert!(!VMCodeMetadata::PAYABLE_BY_SC.is_upgradeable());
        assert!(!VMCodeMetadata::READABLE.is_upgradeable());

        assert!(!VMCodeMetadata::UPGRADEABLE.is_payable());
        assert!(VMCodeMetadata::PAYABLE.is_payable());
        assert!(!VMCodeMetadata::PAYABLE_BY_SC.is_payable());
        assert!(!VMCodeMetadata::READABLE.is_payable());

        assert!(!VMCodeMetadata::UPGRADEABLE.is_payable_by_sc());
        assert!(!VMCodeMetadata::PAYABLE.is_payable_by_sc());
        assert!(VMCodeMetadata::PAYABLE_BY_SC.is_payable_by_sc());
        assert!(!VMCodeMetadata::READABLE.is_payable_by_sc());

        assert!(!VMCodeMetadata::UPGRADEABLE.is_readable());
        assert!(!VMCodeMetadata::PAYABLE.is_readable());
        assert!(!VMCodeMetadata::PAYABLE_BY_SC.is_readable());
        assert!(VMCodeMetadata::READABLE.is_readable());
    }

    /// Translated from vm-wasm.
    #[test]
    fn test_from_array() {
        assert!(VMCodeMetadata::from([1, 0]).is_upgradeable());
        assert!(!VMCodeMetadata::from([1, 0]).is_readable());
        assert!(VMCodeMetadata::from([0, 2]).is_payable());
        assert!(VMCodeMetadata::from([4, 0]).is_readable());
        assert!(!VMCodeMetadata::from([4, 0]).is_upgradeable());
        assert!(!VMCodeMetadata::from([0, 0]).is_upgradeable());
        assert!(!VMCodeMetadata::from([0, 0]).is_payable());
        assert!(!VMCodeMetadata::from([0, 0]).is_readable());
    }
}
