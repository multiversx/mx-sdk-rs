#![allow(clippy::bad_bit_mask)]

use crate::codec::*;
use bitflags::bitflags;

const UPGRADEABLE_STRING: &str = "Upgradeable";
const READABLE_STRING: &str = "Readable";
const GUARDED_STRING: &str = "Guarded";
const PAYABLE_STRING: &str = "Payable";
const PAYABLE_BY_SC_STRING: &str = "PayableBySC";
const DEFAULT_STRING: &str = "Default";

bitflags! {
    /// Bit-flags that govern a smart contract's permissions after deployment.
    ///
    /// Each flag corresponds to a protocol-level capability:
    /// - [`UPGRADEABLE`](CodeMetadata::UPGRADEABLE) — the contract may be upgraded by its owner.
    /// - [`READABLE`](CodeMetadata::READABLE) — other contracts may read this contract's storage.
    /// - [`GUARDED`](CodeMetadata::GUARDED) — transactions require a second guardian signature.
    /// - [`PAYABLE`](CodeMetadata::PAYABLE) — the contract accepts direct EGLD/ESDT transfers.
    /// - [`PAYABLE_BY_SC`](CodeMetadata::PAYABLE_BY_SC) — the contract accepts transfers from other contracts only.
    ///
    /// # Wire format
    ///
    /// Metadata is serialised as exactly **2 big-endian bytes**. The first byte carries
    /// `UPGRADEABLE` (bit 0), `READABLE` (bit 2), and `GUARDED` (bit 3); the second byte carries
    /// `PAYABLE` (bit 1) and `PAYABLE_BY_SC` (bit 2).
    ///
    /// # Parsing
    ///
    /// - Use [`TryFrom<&[u8]>`] / [`TryFrom<u16>`] in **tooling and off-chain code** — these
    ///   return [`CodeMetadataError`] on wrong-length input or unknown bits, enabling fail-closed
    ///   validation.
    /// - Use [`from_bytes_or_default`](CodeMetadata::from_bytes_or_default) in the **Rust VM** —
    ///   this mirrors the lenient Go protocol implementation: wrong-length input yields
    ///   [`EMPTY`](CodeMetadata::EMPTY) and unknown bits are silently truncated.
    ///
    /// # Default value
    ///
    /// [`Default::default()`] returns `UPGRADEABLE | READABLE | PAYABLE | PAYABLE_BY_SC`,
    /// which is the permissive set used as a fallback in **scenario tests** when a deployed
    /// contract's metadata field is left unset. This is intentionally *not* the zero value;
    /// use [`EMPTY`](CodeMetadata::EMPTY) explicitly when you need all flags cleared.
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct CodeMetadata: u16 {
        /// No flags set. The contract is not upgradeable, not readable, and not payable, has no guardian.
        ///
        /// Warning! This is not the default value, see CodeMetadata::default() for that.
        const EMPTY = 0;

        /// The contract can be upgraded in the future.
        const UPGRADEABLE = 0b0000_0001_0000_0000; // LSB of first byte

        /// The contract's storage can be read by other contracts.
        const READABLE = 0b0000_0100_0000_0000; // 3rd LSB of first byte

        /// The contract is guarded: transactions must carry a second signature
        /// from a guardian account in order to be executed.
        const GUARDED = 0b0000_1000_0000_0000; // 4th LSB of first byte

        /// The contract can receive funds without having any endpoint called,
        /// just like user accounts.
        ///
        /// Note: a contract does NOT have to be payable to receive funds
        /// in payable endpoints.
        const PAYABLE = 0b0000_0000_0000_0010; // 2nd LSB of second byte

        /// Like [`CodeMetadata::PAYABLE`], but only allows receiving funds from
        /// other smart contracts. Direct user transfers will be rejected.
        const PAYABLE_BY_SC = 0b0000_0000_0000_0100; // 3rd LSB of second byte
    }
}

impl Default for CodeMetadata {
    fn default() -> Self {
        CodeMetadata::UPGRADEABLE
            | CodeMetadata::READABLE
            | CodeMetadata::PAYABLE
            | CodeMetadata::PAYABLE_BY_SC
    }
}

impl CodeMetadata {
    /// Returns `true` if the contract is allowed to be upgraded.
    pub fn is_upgradeable(&self) -> bool {
        *self & CodeMetadata::UPGRADEABLE != CodeMetadata::EMPTY
    }

    /// Returns `true` if the contract can receive funds without an endpoint call.
    pub fn is_payable(&self) -> bool {
        *self & CodeMetadata::PAYABLE != CodeMetadata::EMPTY
    }

    /// Returns `true` if the contract can receive funds from other smart contracts
    /// without an endpoint call. Direct user transfers are rejected.
    pub fn is_payable_by_sc(&self) -> bool {
        *self & CodeMetadata::PAYABLE_BY_SC != CodeMetadata::EMPTY
    }

    /// Returns `true` if the contract's storage can be read by other contracts.
    pub fn is_readable(&self) -> bool {
        *self & CodeMetadata::READABLE != CodeMetadata::EMPTY
    }

    /// Returns `true` if the contract is guarded.
    pub fn is_guarded(&self) -> bool {
        *self & CodeMetadata::GUARDED != CodeMetadata::EMPTY
    }

    /// Serialises the metadata to its 2-byte big-endian wire representation.
    ///
    /// The returned array is the canonical on-chain encoding: first byte carries
    /// `UPGRADEABLE`, `READABLE`, and `GUARDED`; second byte carries `PAYABLE` and `PAYABLE_BY_SC`.
    #[inline]
    pub fn to_byte_array(&self) -> [u8; 2] {
        self.bits().to_be_bytes()
    }

    /// Serialises the metadata to a 2-byte big-endian `Vec`.
    ///
    /// Convenience wrapper around [`to_byte_array`](Self::to_byte_array) for call sites
    /// that need an owned buffer.
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_byte_array().to_vec()
    }

    /// Calls `f` once for each token that makes up the human-readable display of these flags.
    ///
    /// Tokens are the flag name strings (`"Upgradeable"`, `"Readable"`, `"Guarded"`,
    /// `"Payable"`, `"PayableBySC"`) and the `"|"` separator between them.
    /// If no flags are set, `f` is called once with `"Default"`.
    ///
    /// The flag order matches the canonical display order used in scenario JSON files.
    pub fn write_display_tokens<F: FnMut(&'static str)>(&self, mut write: F) {
        let mut nothing_printed: bool = true;
        if self.is_upgradeable() {
            write(UPGRADEABLE_STRING);
            nothing_printed = false;
        }
        if self.is_readable() {
            if !nothing_printed {
                write("|");
            }
            write(READABLE_STRING);
            nothing_printed = false;
        }
        if self.is_guarded() {
            if !nothing_printed {
                write("|");
            }
            write(GUARDED_STRING);
            nothing_printed = false;
        }
        if self.is_payable() {
            if !nothing_printed {
                write("|");
            }
            write(PAYABLE_STRING);
            nothing_printed = false;
        }
        if self.is_payable_by_sc() {
            if !nothing_printed {
                write("|");
            }
            write(PAYABLE_BY_SC_STRING);
            nothing_printed = false;
        }

        if nothing_printed {
            write(DEFAULT_STRING);
        }
    }

    /// Parses code metadata from a 2-byte slice, mirroring the MultiversX protocol implementation.
    ///
    /// If the slice is not exactly 2 bytes, returns [`CodeMetadata::EMPTY`] (all flags cleared).
    /// Unknown or reserved bits are silently ignored; only the bits corresponding to known flags
    /// are extracted.
    ///
    /// This intentionally lenient behaviour matches the on-chain Go implementation
    /// (`CodeMetadataFromBytes`). Prefer [`TryFrom<&[u8]>`] in tooling and off-chain code
    /// where strict validation is desirable.
    pub fn from_bytes_or_default(bytes: &[u8]) -> Self {
        if bytes.len() != 2 {
            return CodeMetadata::EMPTY;
        }
        let value = u16::from_be_bytes([bytes[0], bytes[1]]);
        CodeMetadata::from_bits_truncate(value)
    }
}

/// Error type returned when converting raw bytes or integers into [`CodeMetadata`] fails.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CodeMetadataError {
    /// The input slice was not exactly 2 bytes long.
    InvalidLength,
    /// The bit pattern contains bits that do not correspond to any known flag.
    InvalidBits(u16),
}

impl TryFrom<u16> for CodeMetadata {
    type Error = CodeMetadataError;

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        CodeMetadata::from_bits(value).ok_or(CodeMetadataError::InvalidBits(value))
    }
}

impl TryFrom<[u8; 2]> for CodeMetadata {
    type Error = CodeMetadataError;

    #[inline]
    fn try_from(arr: [u8; 2]) -> Result<Self, Self::Error> {
        CodeMetadata::try_from(u16::from_be_bytes(arr))
    }
}

impl TryFrom<&[u8]> for CodeMetadata {
    type Error = CodeMetadataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let arr: [u8; 2] = slice
            .try_into()
            .map_err(|_| CodeMetadataError::InvalidLength)?;
        CodeMetadata::try_from(arr)
    }
}

impl TryFrom<&Vec<u8>> for CodeMetadata {
    type Error = CodeMetadataError;

    fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
        CodeMetadata::try_from(v.as_slice())
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
        let value = u16::dep_decode_or_handle_err(input, h)?;
        CodeMetadata::try_from(value).map_err(|_| h.handle_error(DecodeError::INVALID_VALUE))
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
    fn test_empty() {
        assert!(!CodeMetadata::EMPTY.is_upgradeable());
        assert!(!CodeMetadata::EMPTY.is_payable());
        assert!(!CodeMetadata::EMPTY.is_payable_by_sc());
        assert!(!CodeMetadata::EMPTY.is_readable());
        assert!(!CodeMetadata::EMPTY.is_guarded());
    }

    #[test]
    fn test_all() {
        let all = CodeMetadata::UPGRADEABLE
            | CodeMetadata::PAYABLE
            | CodeMetadata::PAYABLE_BY_SC
            | CodeMetadata::READABLE
            | CodeMetadata::GUARDED;
        assert!(all.is_upgradeable());
        assert!(all.is_payable());
        assert!(all.is_payable_by_sc());
        assert!(all.is_readable());
        assert!(all.is_guarded());

        assert_eq!(all.bits(), 0x0d06);

        assert_eq!(CodeMetadata::from_bits_truncate(0xffff), all);
    }

    #[test]
    fn test_each() {
        type CodeMetadataChecker = fn(&CodeMetadata) -> bool;
        let flags_and_checkers: &[(CodeMetadata, CodeMetadataChecker)] = &[
            (CodeMetadata::UPGRADEABLE, CodeMetadata::is_upgradeable),
            (CodeMetadata::READABLE, CodeMetadata::is_readable),
            (CodeMetadata::GUARDED, CodeMetadata::is_guarded),
            (CodeMetadata::PAYABLE, CodeMetadata::is_payable),
            (CodeMetadata::PAYABLE_BY_SC, CodeMetadata::is_payable_by_sc),
        ];

        // checking that checkers match, and are orthogonal (each checker only returns true for its own flag)
        for (flag, checker) in flags_and_checkers {
            for (other_flag, _) in flags_and_checkers {
                assert_eq!(
                    checker(other_flag),
                    flag == other_flag,
                    "{checker:?} returned unexpected result for flag {other_flag:?}",
                );
            }
        }
    }

    #[test]
    fn test_try_from_slice_exact_length() {
        assert!(
            CodeMetadata::try_from(&[1u8, 0u8][..])
                .unwrap()
                .is_upgradeable()
        );
        assert!(
            CodeMetadata::try_from(&[0u8, 2u8][..])
                .unwrap()
                .is_payable()
        );
        assert!(
            !CodeMetadata::try_from(&[0u8, 0u8][..])
                .unwrap()
                .is_upgradeable()
        );
    }

    #[test]
    fn test_try_from_slice_wrong_length() {
        assert_eq!(
            CodeMetadata::try_from(&[1u8][..]),
            Err(CodeMetadataError::InvalidLength)
        );
        assert_eq!(
            CodeMetadata::try_from(&[][..]),
            Err(CodeMetadataError::InvalidLength)
        );
        assert_eq!(
            CodeMetadata::try_from(&[1u8, 0u8, 0u8][..]),
            Err(CodeMetadataError::InvalidLength)
        );
    }

    #[test]
    fn test_try_from_invalid_bits() {
        // 0x0001 has no defined flag in the second byte at bit 0
        assert_eq!(
            CodeMetadata::try_from(0x0001u16),
            Err(CodeMetadataError::InvalidBits(0x0001))
        );
    }

    /// Translated from vm-wasm.
    #[test]
    fn test_try_from_array() {
        assert!(CodeMetadata::try_from([1u8, 0u8]).unwrap().is_upgradeable());
        assert!(!CodeMetadata::try_from([1u8, 0u8]).unwrap().is_readable());
        assert!(!CodeMetadata::try_from([1u8, 0u8]).unwrap().is_guarded());
        assert!(CodeMetadata::try_from([0u8, 2u8]).unwrap().is_payable());
        assert!(CodeMetadata::try_from([4u8, 0u8]).unwrap().is_readable());
        assert!(!CodeMetadata::try_from([4u8, 0u8]).unwrap().is_upgradeable());
        assert!(!CodeMetadata::try_from([4u8, 0u8]).unwrap().is_guarded());
        assert!(CodeMetadata::try_from([8u8, 0u8]).unwrap().is_guarded());
        assert!(!CodeMetadata::try_from([8u8, 0u8]).unwrap().is_upgradeable());
        assert!(!CodeMetadata::try_from([8u8, 0u8]).unwrap().is_readable());
        assert!(!CodeMetadata::try_from([0u8, 0u8]).unwrap().is_upgradeable());
        assert!(!CodeMetadata::try_from([0u8, 0u8]).unwrap().is_payable());
        assert!(!CodeMetadata::try_from([0u8, 0u8]).unwrap().is_readable());
        assert!(!CodeMetadata::try_from([0u8, 0u8]).unwrap().is_guarded());
    }
}
