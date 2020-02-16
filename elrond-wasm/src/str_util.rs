use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

const SEPARATOR: char = '@';

fn from_digit(num: u8) -> char {
    if num < 10 {
        (b'0' + num) as char 
    } else {
        (b'a' + num - 10) as char 
    }
}

pub fn push_i32(s: &mut String, arg: i32) {
    push_i64(s, arg as i64);
}

pub fn push_i64(s: &mut String, arg: i64) {
    s.push(SEPARATOR);
    let mut x = arg;
    if x == 0 {
        s.push('0');
        return;
    }
    if x < 0 {
        s.push('-');
        x = -x;
    }
    let mut temp: Vec<u8> = Vec::with_capacity(8);
    while x > 0 {
        let last_byte: u8 = (x & 0xffi64).try_into().unwrap();
        temp.push(last_byte);
        x = x >> 8;
    }
    for byte in temp.iter().rev() {
        s.push(from_digit(byte>>4));
        s.push(from_digit(byte&0x0f));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use alloc::string::String;

    #[test]
    fn test_push_i32_1() {
        let mut s = String::from("abc");
        push_i32(&mut s, 15);
        assert_eq!(s, "abc@0f");
    }

    #[test]
    fn test_push_i32_2() {
        let mut s = String::from("abc");
        push_i32(&mut s, 256);
        assert_eq!(s, "abc@0100");
    }
}