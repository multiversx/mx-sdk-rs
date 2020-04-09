
use alloc::vec::Vec;
use core::convert::TryInto;

const SEPARATOR: u8 = b'@';

fn from_digit(num: u8) -> u8 {
    if num < 10 {
        b'0' + num
    } else {
        b'a' + num - 10
    }
}

pub struct CallData(Vec<u8>);

impl CallData {
    pub fn new(func_name: &[u8]) -> Self {
        let mut data = Vec::with_capacity(func_name.len());
        data.extend_from_slice(func_name);
        CallData(data)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn push_empty(&mut self) {
        self.0.push(SEPARATOR);
    }

    fn push_byte(&mut self, byte: u8) {
        self.0.push(from_digit(byte>>4));
        self.0.push(from_digit(byte&0x0f));
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.0.push(SEPARATOR);
        for byte in bytes.iter() {
            self.push_byte(*byte);
        }
    }

    pub fn push_i32(&mut self, arg: i32) {
        self.push_i64(arg as i64);
    }

    pub fn push_i64(&mut self, arg: i64) {
        self.0.push(SEPARATOR);
        let mut x = arg;
        if x == 0 {
            self.0.push(b'0');
            return;
        }
        if x < 0 {
            self.0.push(b'-');
            x = -x;
        }
        let mut temp: Vec<u8> = Vec::with_capacity(8);
        while x > 0 {
            let last_byte: u8 = (x & 0xffi64).try_into().unwrap();
            temp.push(last_byte);
            x = x >> 8;
        }
        for byte in temp.iter().rev() {
            self.push_byte(*byte);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_i32_1() {
        let mut cd = CallData::new(&*b"abc");
        cd.push_i32(15);
        assert_eq!(cd.as_slice(), *b"abc@0f");
    }

    #[test]
    fn test_push_i32_2() {
        let mut cd = CallData::new(&*b"abc");
        cd.push_i32(256);
        assert_eq!(cd.as_slice(), *b"abc@0100");
    }
}