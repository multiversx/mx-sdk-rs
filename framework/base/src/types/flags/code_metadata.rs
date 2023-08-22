#![allow(clippy::bad_bit_mask)]

use crate::{
    abi::{TypeAbi, TypeName},
    codec::*,
    formatter::{hex_util, FormatByteReceiver, SCBinary, SCDisplay, SCLowerHex},
};
use bitflags::bitflags;

const UPGRADEABLE_STRING: &[u8] = b"Upgradeable";
const READABLE_STRING: &[u8] = b"Readable";
const PAYABLE_STRING: &[u8] = b"Payable";
const PAYABLE_BY_SC_STRING: &[u8] = b"PayableBySC";
const DEFAULT_STRING: &[u8] = b"Default";

bitflags! {
    #[derive(Default)]
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

impl TypeAbi for CodeMetadata {
    fn type_name() -> TypeName {
        "CodeMetadata".into()
    }
}

impl SCDisplay for CodeMetadata {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let mut nothing_printed: bool = true;
        verify_metadata_and_append(
            self.is_upgradeable(),
            f,
            UPGRADEABLE_STRING,
            &mut nothing_printed,
        );
        verify_metadata_and_append(self.is_readable(), f, READABLE_STRING, &mut nothing_printed);
        verify_metadata_and_append(self.is_payable(), f, PAYABLE_STRING, &mut nothing_printed);
        verify_metadata_and_append(
            self.is_payable_by_sc(),
            f,
            PAYABLE_BY_SC_STRING,
            &mut nothing_printed,
        );
        if nothing_printed {
            f.append_bytes(DEFAULT_STRING);
        }
    }
}

impl SCLowerHex for CodeMetadata {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let num = self.bits().to_be_bytes();
        f.append_bytes(&hex_util::byte_to_hex_digits(num[0])[..]);
        f.append_bytes(&hex_util::byte_to_hex_digits(num[1])[..]);
    }
}

impl SCBinary for CodeMetadata {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let num = self.bits().to_be_bytes();
        f.append_bytes(&hex_util::byte_to_binary_digits(num[0])[..]);
        f.append_bytes(&hex_util::byte_to_binary_digits(num[1])[..]);
    }
}

fn verify_metadata_and_append<F: FormatByteReceiver>(
    constraint: bool,
    f: &mut F,
    bytes_to_append: &[u8],
    nothing_printed: &mut bool,
) {
    if constraint {
        if !*nothing_printed {
            f.append_bytes(b"|");
        }
        f.append_bytes(bytes_to_append);
        *nothing_printed = false;
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
