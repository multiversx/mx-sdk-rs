use alloc::string::String;

use crate::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

const HEX_VALUE_PREFIX: &[u8] = b"0x";
const STATIC_BUFFER_LEN: usize = 10;

fn half_byte_to_hex_digit(num: u8) -> u8 {
    if num < 10 {
        b'0' + num
    } else {
        b'a' + num - 0xau8
    }
}

pub fn byte_to_hex_digits(byte: u8) -> (u8, u8) {
    let digit1 = half_byte_to_hex_digit(byte >> 4);
    let digit2 = half_byte_to_hex_digit(byte & 0x0f);
    (digit1, digit2)
}

pub fn encode_bytes_as_hex(input: &[u8]) -> String {
    let mut result = String::new();
    for &byte in input {
        let (first_byte, second_byte) = byte_to_hex_digits(byte);
        result.push(first_byte as char);
        result.push(second_byte as char);
    }
    result
}

fn hex_digit_to_half_byte(digit: u8) -> Option<u8> {
    if (b'0'..=b'9').contains(&digit) {
        return Some(digit - b'0');
    }
    if (b'a'..=b'f').contains(&digit) {
        return Some(digit - b'a' + 0xau8);
    }
    None
}

pub fn hex_digits_to_byte(digit1: u8, digit2: u8) -> Option<u8> {
    let mut result = match hex_digit_to_half_byte(digit1) {
        None => {
            return None;
        },
        Some(num) => num << 4,
    };
    match hex_digit_to_half_byte(digit2) {
        None => {
            return None;
        },
        Some(num) => {
            result |= num;
        },
    };
    Some(result)
}

pub fn add_arg_as_hex_to_buffer<M: ManagedTypeApi>(
    buffer: &mut ManagedBufferCachedBuilder<M>,
    arg: &ManagedBuffer<M>,
) {
    buffer.append_bytes(HEX_VALUE_PREFIX);

    let arg_len = arg.len();
    if arg_len == 0 {
        return;
    }

    let mut static_buffer = [0u8; STATIC_BUFFER_LEN];
    let mut hex_bytes_buffer = [0u8; STATIC_BUFFER_LEN * 2];

    let mut current_arg_index = 0;
    while current_arg_index < arg_len {
        let bytes_remaining = arg_len - current_arg_index;
        let bytes_to_load = core::cmp::min(bytes_remaining, STATIC_BUFFER_LEN);

        let slice = &mut static_buffer[0..bytes_to_load];
        let _ = arg.load_slice(current_arg_index, slice);

        for i in 0..bytes_to_load {
            let (hex1, hex2) = byte_to_hex_digits(slice[i]);
            hex_bytes_buffer[i * 2] = hex1;
            hex_bytes_buffer[i * 2 + 1] = hex2;
        }

        let hex_slice = &hex_bytes_buffer[0..(bytes_to_load * 2)];
        buffer.append_bytes(hex_slice);

        current_arg_index += STATIC_BUFFER_LEN;
    }
}
