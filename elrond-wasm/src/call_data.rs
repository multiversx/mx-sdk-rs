
use alloc::vec::Vec;
use super::address::*;
use super::err_msg;

const SEPARATOR: u8 = b'@';

fn to_single_digit(num: u8) -> u8 {
    if num < 10 {
        b'0' + num
    } else {
        b'a' + num - 0xau8
    }
}

fn serialize_byte(byte: u8) -> (u8, u8) {
    let digit1 = to_single_digit(byte>>4);
    let digit2 = to_single_digit(byte&0x0f);
    (digit1, digit2)
}

fn from_single_digit(digit: u8) -> Option<u8> {
    if digit >= b'0' && digit <= b'9' {
        return Some(digit - b'0');
    }
    if digit >= b'a' && digit <= b'f' {
        return Some(digit - b'a' + 0xau8);
    }
    None
}

fn deserialize_byte(digit1: u8, digit2: u8) -> Option<u8> {
    let mut result: u8;
    match from_single_digit(digit1) {
        None => { return None; },
        Some(num) => {
            result = num << 4;
        }
    };
    match from_single_digit(digit2) {
        None => { return None; },
        Some(num) => {
            result |= num;
        }
    };
    Some(result)
}

pub struct CallData(Vec<u8>);

/// Serialization part.
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
        let (digit1, digit2) = serialize_byte(byte);
        self.0.push(digit1);
        self.0.push(digit2);
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.0.push(SEPARATOR);
        for byte in bytes.iter() {
            self.push_byte(*byte);
        }
    }

    #[inline]
    pub fn push_i32(&mut self, arg: i32) {
        self.push_u32(arg as u32);
    }

    pub fn push_u32(&mut self, arg: u32) {
        self.0.push(SEPARATOR);
        self.push_byte((arg >> 24 & 0xff) as u8);
        self.push_byte((arg >> 16 & 0xff) as u8);
        self.push_byte((arg >>  8 & 0xff) as u8);
        self.push_byte((arg       & 0xff) as u8);
    }

    #[inline]
    pub fn push_i64(&mut self, arg: i64) {
        self.push_u64(arg as u64);
    }

    pub fn push_u64(&mut self, arg: u64) {
        self.0.push(SEPARATOR);
        self.push_byte((arg >> 56 & 0xff) as u8);
        self.push_byte((arg >> 48 & 0xff) as u8);
        self.push_byte((arg >> 40 & 0xff) as u8);
        self.push_byte((arg >> 32 & 0xff) as u8);
        self.push_byte((arg >> 24 & 0xff) as u8);
        self.push_byte((arg >> 16 & 0xff) as u8);
        self.push_byte((arg >>  8 & 0xff) as u8);
        self.push_byte((arg       & 0xff) as u8);
    }
}

pub struct CallDataDeserializer<'a> {
    data: &'a CallData,
    index: usize,
}

/// Deserialization part.
impl CallData {
    pub fn from_raw_data(raw_data: Vec<u8>) -> Self {
        CallData(raw_data)
    }

    pub fn deserializer<'a>(&'a self) -> CallDataDeserializer<'a> {
        CallDataDeserializer {
            data: &self,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeserializerResult<T, E> {
    Res(T),
    NoMore,
    Err(E),
}

use DeserializerResult::*;

impl<'a> CallDataDeserializer<'a> {
    pub fn next_raw_bytes(&mut self) -> DeserializerResult<&'a [u8], &str> {
        let initial_index = self.index;
        loop {
            if self.index > self.data.0.len() {
                return NoMore;
            }

            if self.index == self.data.0.len() {
                let slice = &self.data.0[initial_index..self.index];
                self.index += 1; // make index = len + 1 to signal that we are done, and return NoMore from the next call on
                return Res(slice);
            }
            
            let c = self.data.0[self.index];
            if c == SEPARATOR {
                let slice = &self.data.0[initial_index..self.index];
                self.index += 1;
                return Res(slice);
            }

            self.index += 1;
        }
    }

    pub fn next_h256(&mut self) -> DeserializerResult<H256, &str> {
        match self.next_raw_bytes() {
            NoMore => NoMore,
            Err(e) => Err(e),
            Res(data_raw) => {
                if data_raw.len() != 64 {
                    return Err(err_msg::DESERIALIZATION_NOT_32_BYTES);
                }
                let mut arr = [0u8; 32];
                for i in 0..32 {
                    match deserialize_byte(data_raw[2*i], data_raw[2*i+1]) {
                        None => {
                            return Err(err_msg::DESERIALIZATION_INVALID_BYTE);
                        },
                        Some(byte) => {
                            arr[i] = byte;
                        }
                    }
                }
                Res(arr.into())
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_i32_1() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i32(15);
        assert_eq!(cd.as_slice(), &b"func@0000000f"[..]);
    }

    #[test]
    fn test_push_i32_2() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i32(256);
        assert_eq!(cd.as_slice(), &b"func@00000100"[..]);
    }

    #[test]
    fn test_push_i32_3() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i32(-1);
        assert_eq!(cd.as_slice(), &b"func@ffffffff"[..]);
    }

    #[test]
    fn test_push_i32_4() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i32(0x12345678);
        assert_eq!(cd.as_slice(), &b"func@12345678"[..]);
    }

    #[test]
    fn test_push_i64_1() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i64(15);
        assert_eq!(cd.as_slice(), &b"func@000000000000000f"[..]);
    }

    #[test]
    fn test_push_i64_2() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i64(256);
        assert_eq!(cd.as_slice(), &b"func@0000000000000100"[..]);
    }

    #[test]
    fn test_push_i64_3() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i64(-1);
        assert_eq!(cd.as_slice(), &b"func@ffffffffffffffff"[..]);
    }

    #[test]
    fn test_push_i64_4() {
        let mut cd = CallData::new(&*b"func");
        cd.push_i64(0x0123456789abcdef);
        assert_eq!(cd.as_slice(), &b"func@0123456789abcdef"[..]);
    }

    #[test]
    fn test_next_raw_bytes_1() {
        let cd = CallData::from_raw_data((&b"func@1111@2222").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&b"func"[..]));
        assert_eq!(de.next_raw_bytes(), Res(&b"1111"[..]));
        assert_eq!(de.next_raw_bytes(), Res(&b"2222"[..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_empty() {
        let cd = CallData::from_raw_data(Vec::with_capacity(0));
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_only_func() {
        let cd = CallData::from_raw_data((&b"func").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&b"func"[..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_some_empty() {
        let cd = CallData::from_raw_data((&b"func@@2222").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&b"func"[..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&b"2222"[..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_ends_empty() {
        let cd = CallData::from_raw_data((&b"func@").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&b"func"[..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_many_empty() {
        let cd = CallData::from_raw_data((&b"func@@2222@@").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&b"func"[..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&b"2222"[..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_all_empty() {
        let cd = CallData::from_raw_data((&b"@@@").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_raw_bytes_all_empty_but_last() {
        let cd = CallData::from_raw_data((&b"@@@123").to_vec());
        let mut de = cd.deserializer();
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&[][..]));
        assert_eq!(de.next_raw_bytes(), Res(&b"123"[..]));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }

    #[test]
    fn test_next_h256() {
        let cd = CallData::from_raw_data((&b"func@0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef").to_vec());
        let mut de = cd.deserializer();
        let expected: [u8; 32] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                                  0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                                  0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                                  0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef
                                 ];
        assert_eq!(de.next_raw_bytes(), Res(&b"func"[..]));
        assert!(de.next_h256() == Res(Address::from(expected)));
        assert_eq!(de.next_raw_bytes(), NoMore);
        assert_eq!(de.next_raw_bytes(), NoMore);
    }
}