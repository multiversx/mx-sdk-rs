use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

fn from_digit(num: u8) -> char {
    if num < 10 {
        (b'0' + num) as char 
    } else {
        (b'a' + num - 10) as char 
    }
}

pub fn push_i32(s: &mut String, arg: i32) {
    let mut x = arg;
    if x == 0 {
        s.push('0');
        return;
    }
    if x < 0 {
        s.push('-');
        x = -x;
    }
    let mut temp: Vec<u8> = Vec::with_capacity(4);
    while x > 0 {
        let last_byte: u8 = (x & 0xffi32).try_into().unwrap();
        temp.push(last_byte);
        x = x >> 8;
    }
    for byte in temp.iter().rev() {
        s.push(from_digit(*byte));
    }
}

pub fn push_i64(s: &mut String, arg: i64) {
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
        s.push(from_digit(*byte));
    }
}