use core::fmt::Display;

use elrond_wasm::{formatter::*, types::CodeMetadata};

#[derive(Default)]
struct SimpleReceiver(String);

impl FormatByteReceiver for SimpleReceiver {
    fn append_bytes(&mut self, bytes: &[u8]) {
        self.0.push_str(core::str::from_utf8(bytes).unwrap());
    }

    fn append_managed_buffer<M: elrond_wasm::api::ManagedTypeApi>(
        &mut self,
        _item: &elrond_wasm::types::ManagedBuffer<M>,
    ) {
        unimplemented!()
    }

    fn append_managed_buffer_lower_hex<M: elrond_wasm::api::ManagedTypeApi>(
        &mut self,
        _item: &elrond_wasm::types::ManagedBuffer<M>,
    ) {
        unimplemented!()
    }

    fn append_managed_buffer_binary<M: elrond_wasm::api::ManagedTypeApi>(
        &mut self,
        _item: &elrond_wasm::types::ManagedBuffer<M>,
    ) {
        unimplemented!()
    }
}

/// Expects that the output from SCDisplay is the same as the standard Rust display.
fn check_display<T: SCDisplay + Display>(item: T) {
    let mut receiver = SimpleReceiver::default();
    SCDisplay::fmt(&item, &mut receiver);
    let expected = format!("{}", item);
    assert_eq!(receiver.0, expected);
}

/// Expects that the output from SCLowerHex is the same as the standard Rust display.
fn check_lower_hex<T: SCLowerHex + Display + std::fmt::LowerHex>(item: T) {
    let mut receiver = SimpleReceiver::default();
    SCLowerHex::fmt(&item, &mut receiver);
    let expected = format!("{:x}", item);
    assert_eq!(receiver.0, expected);
}

fn check_code_metadata_display<T: SCDisplay, D: Display>(item: T, expected: D) {
    let mut receiver = SimpleReceiver::default();
    SCDisplay::fmt(&item, &mut receiver);
    assert_eq!(receiver.0, expected.to_string());
}

#[test]
fn test_lower_hex_usigned() {
    check_lower_hex(0x0u8);
    check_lower_hex(0x50u8);
    check_lower_hex(0xFFu8);
    check_lower_hex(0x0u16);
    check_lower_hex(0x50u16);
    check_lower_hex(0xFFFFu16);
    check_lower_hex(0x0u32);
    check_lower_hex(0x50u32);
    check_lower_hex(0xFFFFFFFFu32);
    check_lower_hex(0x50u32);
    check_lower_hex(0x0u64);
    check_lower_hex(0x5u64);
    check_lower_hex(0x50u64);
    check_lower_hex(u64::MAX);
}

#[test]
fn test_lower_hex_signed() {
    check_lower_hex(0xai32);
    check_lower_hex(5i32);
    check_lower_hex(50i32);
    check_lower_hex(-2i32);
    check_lower_hex(-1i16);
    check_lower_hex(-50i32);
    check_lower_hex(-100i32);
    check_lower_hex(-0xFFi32);
    check_lower_hex(-0xFFFFi32);
    check_lower_hex(-0x01i64);
    check_lower_hex(-0x50i64);
    check_lower_hex(-0xFFi64);
    check_lower_hex(-0xFFFFi64);
    check_lower_hex(-0xFFFFFFFFi64);
    check_lower_hex(i8::MAX);
    check_lower_hex(i8::MIN);
    check_lower_hex(i16::MAX);
    check_lower_hex(i16::MIN);
    check_lower_hex(i32::MAX);
    check_lower_hex(i32::MIN);
    check_lower_hex(i64::MAX);
    check_lower_hex(i64::MIN);
}

#[test]
fn test_display_usigned() {
    check_display(0u8);
    check_display(50u8);
    check_display(0u16);
    check_display(50u16);
    check_display(0u32);
    check_display(5u32);
    check_display(50u32);
    check_display(0u64);
    check_display(5u64);
    check_display(50u64);
    check_display(u64::MAX);
}

#[test]
fn test_format_signed() {
    check_display(0i32);
    check_display(5i32);
    check_display(50i32);
    check_display(-1i32);
    check_display(-50i32);
    check_display(-100i32);
    check_display(i8::MAX);
    check_display(i8::MIN);
    check_display(i16::MAX);
    check_display(i16::MIN);
    check_display(i32::MAX);
    check_display(i32::MIN);
    check_display(i64::MAX);
    check_display(i64::MIN);
}

#[test]
fn test_display_code_metadata() {
    check_code_metadata_display(CodeMetadata::UPGRADEABLE, "Upgradeable");
    check_code_metadata_display(CodeMetadata::READABLE, "Readable");
    check_code_metadata_display(CodeMetadata::PAYABLE, "Payable");
    check_code_metadata_display(CodeMetadata::PAYABLE_BY_SC, "PayableBySC");
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE,
        "Upgradeable|Readable",
    );
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE | CodeMetadata::PAYABLE,
        "Upgradeable|Payable",
    );
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE | CodeMetadata::PAYABLE_BY_SC,
        "Upgradeable|PayableBySC",
    );
    check_code_metadata_display(
        CodeMetadata::READABLE | CodeMetadata::PAYABLE,
        "Readable|Payable",
    );
    check_code_metadata_display(
        CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
        "Readable|PayableBySC",
    );
    check_code_metadata_display(
        CodeMetadata::PAYABLE | CodeMetadata::PAYABLE_BY_SC,
        "Payable|PayableBySC",
    );
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE,
        "Upgradeable|Readable|Payable",
    );
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
        "Upgradeable|Readable|PayableBySC",
    );
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE | CodeMetadata::PAYABLE | CodeMetadata::PAYABLE_BY_SC,
        "Upgradeable|Payable|PayableBySC",
    );
    check_code_metadata_display(
        CodeMetadata::READABLE | CodeMetadata::PAYABLE | CodeMetadata::PAYABLE_BY_SC,
        "Readable|Payable|PayableBySC",
    );
    check_code_metadata_display(
        CodeMetadata::UPGRADEABLE
            | CodeMetadata::PAYABLE
            | CodeMetadata::READABLE
            | CodeMetadata::PAYABLE_BY_SC,
        "Upgradeable|Readable|Payable|PayableBySC",
    );
    check_code_metadata_display(CodeMetadata::DEFAULT, "Default");
}
