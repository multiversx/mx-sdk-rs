use core::fmt::Display;

use elrond_wasm::formatter::*;

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
}

/// Expects that the output from SCDisplay is the same as the standard Rust display.
fn check_display<T: SCDisplay + Display>(item: T) {
    let mut receiver = SimpleReceiver::default();
    SCDisplay::fmt(&item, &mut receiver);
    let expected = format!("{}", item);
    assert_eq!(receiver.0, expected);
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
