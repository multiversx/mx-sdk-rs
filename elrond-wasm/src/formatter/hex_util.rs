use alloc::string::String;

pub fn byte_to_bin_digit(mut num: u8) -> [u8; 8] {
    let mut result = [0u8; 8];
    let mut i = 0;
    while num > 0 {
        result[i] += &(num % 2);
        num /= 2;
        i += 1;
    }
    result
}

fn half_byte_to_hex_digit(num: u8) -> u8 {
    if num < 10 {
        b'0' + num
    } else {
        b'a' + num - 0xau8
    }
}

pub fn byte_to_hex_digits(byte: u8) -> [u8; 2] {
    let digit1 = half_byte_to_hex_digit(byte >> 4);
    let digit2 = half_byte_to_hex_digit(byte & 0x0f);
    [digit1, digit2]
}

pub fn encode_bytes_as_hex(input: &[u8]) -> String {
    let mut result = String::new();
    for &byte in input {
        let bytes = byte_to_hex_digits(byte);
        result.push(bytes[0] as char);
        result.push(bytes[1] as char);
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
