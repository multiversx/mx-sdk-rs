#![allow(clippy::bad_bit_mask)]

use crate::codec::*;
use bitflags::bitflags;

const UPGRADEABLE_STRING: &str = "Upgradeable";
const READABLE_STRING: &str = "Readable";
const PAYABLE_STRING: &str = "Payable";
const PAYABLE_BY_SC_STRING: &str = "PayableBySC";
const DEFAULT_STRING: &str = "Default";

bitflags! {
    #[derive(Default, PartialEq, Debug, Clone, Copy)]
    pub struct CodeMetadata: u16 {
        const DEFAULT = 0;
        const UPGRADEABLE = 0b0000_0001_0000_0000; // LSB of first byte
        const READABLE = 0b0000_0100_0000_0000; // 3rd LSB of first byte
        const PAYABLE = 0b0000_0000_0000_0010; // 2nd LSB of second byte
        const PAYABLE_BY_SC = 0b0000_0000_0000_0100; // 3rd LSB of second byte
    }
}

impl CodeMetadata {
    pub fn is_upgradeable(&self) -> bool {
        *self & CodeMetadata::UPGRADEABLE != CodeMetadata::DEFAULT
    }

    pub fn is_payable(&self) -> bool {
        *self & CodeMetadata::PAYABLE != CodeMetadata::DEFAULT
    }

    pub fn is_payable_by_sc(&self) -> bool {
        *self & CodeMetadata::PAYABLE_BY_SC != CodeMetadata::DEFAULT
    }

    pub fn is_readable(&self) -> bool {
        *self & CodeMetadata::READABLE != CodeMetadata::DEFAULT
    }

    #[inline]
    pub fn to_byte_array(&self) -> [u8; 2] {
        self.bits().to_be_bytes()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.to_byte_array().to_vec()
    }

    pub fn for_each_string_token<F: FnMut(&'static str)>(&self, mut f: F) {
        let mut nothing_printed: bool = true;
        if self.is_upgradeable() {
            f(UPGRADEABLE_STRING);
            nothing_printed = false;
        }
        if self.is_readable() {
            if !nothing_printed {
                f("|");
            }
            f(READABLE_STRING);
            nothing_printed = false;
        }
        if self.is_payable() {
            if !nothing_printed {
                f("|");
            }
            f(PAYABLE_STRING);
            nothing_printed = false;
        }
        if self.is_payable_by_sc() {
            if !nothing_printed {
                f("|");
            }
            f(PAYABLE_BY_SC_STRING);
            nothing_printed = false;
        }

        if nothing_printed {
            f(DEFAULT_STRING);
        }
    }
}

impl From<[u8; 2]> for CodeMetadata {
    #[inline]
    fn from(arr: [u8; 2]) -> Self {
        CodeMetadata::from(u16::from_be_bytes(arr))
    }
}

impl From<u16> for CodeMetadata {
    #[inline]
    fn from(value: u16) -> Self {
        CodeMetadata::from_bits_truncate(value)
    }
}

impl From<&[u8]> for CodeMetadata {
    fn from(slice: &[u8]) -> Self {
        let arr: [u8; 2] = slice.try_into().unwrap_or_default();
        CodeMetadata::from(arr)
    }
}

impl From<&Vec<u8>> for CodeMetadata {
    fn from(v: &Vec<u8>) -> Self {
        CodeMetadata::from(v.as_slice())
    }
}

impl NestedEncode for CodeMetadata {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.bits().dep_encode_or_handle_err(dest, h)?;
        Ok(())
    }
}

impl TopEncode for CodeMetadata {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for CodeMetadata {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(CodeMetadata::from(u16::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for CodeMetadata {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert!(!CodeMetadata::DEFAULT.is_upgradeable());
        assert!(!CodeMetadata::DEFAULT.is_payable());
        assert!(!CodeMetadata::DEFAULT.is_readable());
    }

    #[test]
    fn test_all() {
        let all = CodeMetadata::UPGRADEABLE
            | CodeMetadata::PAYABLE
            | CodeMetadata::PAYABLE_BY_SC
            | CodeMetadata::READABLE;
        assert!(all.is_upgradeable());
        assert!(all.is_payable());
        assert!(all.is_payable_by_sc());
        assert!(all.is_readable());

        assert_eq!(all.bits(), 0x0506);

        assert_eq!(CodeMetadata::from_bits_truncate(0xffff), all);
    }

    #[test]
    fn test_each() {
        assert!(CodeMetadata::UPGRADEABLE.is_upgradeable());
        assert!(!CodeMetadata::PAYABLE.is_upgradeable());
        assert!(!CodeMetadata::PAYABLE_BY_SC.is_upgradeable());
        assert!(!CodeMetadata::READABLE.is_upgradeable());

        assert!(!CodeMetadata::UPGRADEABLE.is_payable());
        assert!(CodeMetadata::PAYABLE.is_payable());
        assert!(!CodeMetadata::PAYABLE_BY_SC.is_payable());
        assert!(!CodeMetadata::READABLE.is_payable());

        assert!(!CodeMetadata::UPGRADEABLE.is_payable_by_sc());
        assert!(!CodeMetadata::PAYABLE.is_payable_by_sc());
        assert!(CodeMetadata::PAYABLE_BY_SC.is_payable_by_sc());
        assert!(!CodeMetadata::READABLE.is_payable_by_sc());

        assert!(!CodeMetadata::UPGRADEABLE.is_readable());
        assert!(!CodeMetadata::PAYABLE.is_readable());
        assert!(!CodeMetadata::PAYABLE_BY_SC.is_readable());
        assert!(CodeMetadata::READABLE.is_readable());
    }

    /// Translated from vm-wasm.
    #[test]
    fn test_from_array() {
        assert!(CodeMetadata::from([1, 0]).is_upgradeable());
        assert!(!CodeMetadata::from([1, 0]).is_readable());
        assert!(CodeMetadata::from([0, 2]).is_payable());
        assert!(CodeMetadata::from([4, 0]).is_readable());
        assert!(!CodeMetadata::from([4, 0]).is_upgradeable());
        assert!(!CodeMetadata::from([0, 0]).is_upgradeable());
        assert!(!CodeMetadata::from([0, 0]).is_payable());
        assert!(!CodeMetadata::from([0, 0]).is_readable());
    }
}
