use crate::abi::TypeAbi;
use alloc::string::String;
use core::ops::{BitOr, BitOrAssign};
use elrond_codec::*;

/// Flags concerning smart contract creation and upgrade.
/// Currently always represented as a 2-byte bitfield.
#[derive(Clone, Copy, PartialEq)]
pub struct CodeMetadata([u8; 2]);

const METADATA_UPGRADEABLE_BYTE: usize = 0;
const METADATA_UPGRADEABLE_MASK: u8 = 1;
const METADATA_PAYABLE_BYTE: usize = 1;
const METADATA_PAYABLE_MASK: u8 = 2;
const METADATA_READABLE_BYTE: usize = 0;
const METADATA_READABLE_MASK: u8 = 4;

impl CodeMetadata {
    pub const DEFAULT: CodeMetadata = CodeMetadata([0, 0]);
    pub const UPGRADEABLE: CodeMetadata = CodeMetadata([METADATA_UPGRADEABLE_MASK, 0]);
    pub const PAYABLE: CodeMetadata = CodeMetadata([0, METADATA_PAYABLE_MASK]);
    pub const READABLE: CodeMetadata = CodeMetadata([METADATA_READABLE_MASK, 0]);

    pub fn is_upgradeable(&self) -> bool {
        self.0[METADATA_UPGRADEABLE_BYTE] & METADATA_UPGRADEABLE_MASK > 0
    }

    pub fn is_payable(&self) -> bool {
        self.0[METADATA_PAYABLE_BYTE] & METADATA_PAYABLE_MASK > 0
    }

    pub fn is_readable(&self) -> bool {
        self.0[METADATA_READABLE_BYTE] & METADATA_READABLE_MASK > 0
    }

    pub fn from_flags(upgradeable: bool, payable: bool, readable: bool) -> CodeMetadata {
        let mut code_metadata = CodeMetadata::DEFAULT;
        if upgradeable {
            code_metadata |= CodeMetadata::UPGRADEABLE;
        }
        if payable {
            code_metadata |= CodeMetadata::PAYABLE;
        }
        if readable {
            code_metadata |= CodeMetadata::READABLE;
        }
        code_metadata
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0[..].as_ptr()
    }

    #[inline]
    pub fn into_bytes(self) -> [u8; 2] {
        self.0
    }

    pub fn to_u16(&self) -> u16 {
        u16::from_be_bytes(self.0)
    }
}

impl From<[u8; 2]> for CodeMetadata {
    #[inline]
    fn from(arr: [u8; 2]) -> Self {
        CodeMetadata(arr)
    }
}

impl From<u16> for CodeMetadata {
    #[inline]
    fn from(value: u16) -> Self {
        CodeMetadata(value.to_be_bytes())
    }
}

impl BitOr for CodeMetadata {
    type Output = CodeMetadata;

    fn bitor(self, other: CodeMetadata) -> CodeMetadata {
        CodeMetadata([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }
}

impl<'a, 'b> BitOr<&'b CodeMetadata> for &'a CodeMetadata {
    type Output = CodeMetadata;

    fn bitor(self, other: &CodeMetadata) -> CodeMetadata {
        CodeMetadata([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }
}

impl BitOrAssign<CodeMetadata> for CodeMetadata {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0[0] |= other.0[0];
        self.0[1] |= other.0[1];
    }
}

impl BitOrAssign<&CodeMetadata> for CodeMetadata {
    #[inline]
    fn bitor_assign(&mut self, other: &CodeMetadata) {
        self.0[0] |= other.0[0];
        self.0[1] |= other.0[1];
    }
}

impl NestedEncode for CodeMetadata {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.to_u16().dep_encode(dest)?;
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.to_u16().dep_encode_or_exit(dest, c, exit);
    }
}

impl TopEncode for CodeMetadata {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}

impl NestedDecode for CodeMetadata {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(CodeMetadata::from(u16::dep_decode(input)?))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        CodeMetadata::from(u16::dep_decode_or_exit(input, c, exit))
    }
}

impl TopDecode for CodeMetadata {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        top_decode_from_nested(input)
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        top_decode_from_nested_or_exit(input, c, exit)
    }
}

impl TypeAbi for CodeMetadata {
    fn type_name() -> String {
        "CodeMetadata".into()
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
    fn test_const() {
        assert!(CodeMetadata::UPGRADEABLE.is_upgradeable());
        assert!(CodeMetadata::PAYABLE.is_payable());
        assert!(CodeMetadata::READABLE.is_readable());
    }

    #[test]
    fn test_all() {
        let all = CodeMetadata::UPGRADEABLE | CodeMetadata::PAYABLE | CodeMetadata::READABLE;
        assert!(all.is_upgradeable());
        assert!(all.is_payable());
        assert!(all.is_readable());
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

    #[test]
    fn test_from_flags() {
        assert!(CodeMetadata::from_flags(true, false, false).is_upgradeable());
        assert!(CodeMetadata::from_flags(false, true, false).is_payable());
        assert!(CodeMetadata::from_flags(false, false, true).is_readable());
        assert!(!CodeMetadata::from_flags(false, false, false).is_upgradeable());
        assert!(!CodeMetadata::from_flags(false, false, false).is_payable());
        assert!(!CodeMetadata::from_flags(false, false, false).is_readable());

        assert_eq!(CodeMetadata::from_flags(true, true, true).to_u16(), 0x0502);
    }
}
