
use alloc::vec::Vec;

use super::SEPARATOR;

fn half_byte_to_hex_digit(num: u8) -> u8 {
    if num < 10 {
        b'0' + num
    } else {
        b'a' + num - 0xau8
    }
}

fn byte_to_hex(byte: u8) -> (u8, u8) {
    let digit1 = half_byte_to_hex_digit(byte>>4);
    let digit2 = half_byte_to_hex_digit(byte&0x0f);
    (digit1, digit2)
}

pub struct CallDataSerializer(Vec<u8>);

/// Serializes to Elrond's smart contract call format.
/// 
/// This format consists of the function name, followed by '@', follwed by hex-encoded argument bytes separated by '@' characters.
/// Example: "funcName@00000@aaaa@1234@@".
/// Arguments can be empty, in which case no hex digits are emitted.
/// Argument hex encodings will always have an even number of digits.
/// 
/// CallDataSerializer owns its output.
/// 
/// Converting from whatever type the argument to bytes is not in scope. Use the `serializer` module for that.
/// 
impl CallDataSerializer {
    pub fn new(func_name: &[u8]) -> Self {
        let mut data = Vec::with_capacity(func_name.len());
        data.extend_from_slice(func_name);
        CallDataSerializer(data)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn push_byte(&mut self, byte: u8) {
        let (digit1, digit2) = byte_to_hex(byte);
        self.0.push(digit1);
        self.0.push(digit2);
    }

    pub fn push_argument_bytes(&mut self, bytes: &[u8]) {
        self.0.reserve(1 + bytes.len() * 2);
        self.0.push(SEPARATOR);
        for byte in bytes.iter() {
            self.push_byte(*byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_bytes_1() {
        let mut cd = CallDataSerializer::new(&*b"func");
        let arg_bytes: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        cd.push_argument_bytes(arg_bytes);
        assert_eq!(cd.as_slice(), &b"func@0123456789abcdef"[..]);
    }

    #[test]
    fn test_push_bytes_2() {
        let mut cd = CallDataSerializer::new(&*b"func");
        let arg_bytes: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        cd.push_argument_bytes(arg_bytes);
        cd.push_argument_bytes(arg_bytes);
        assert_eq!(cd.as_slice(), &b"func@0123456789abcdef@0123456789abcdef"[..]);
    }

    #[test]
    fn test_push_empty_1() {
        let mut cd = CallDataSerializer::new(&*b"func");
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@"[..]);
    }

    #[test]
    fn test_push_empty_2() {
        let mut cd = CallDataSerializer::new(&*b"func");
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@@"[..]);
    }

    #[test]
    fn test_push_empty_3() {
        let mut cd = CallDataSerializer::new(&*b"");
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"@@@"[..]);
    }

    #[test]
    fn test_push_some_empty_1() {
        let mut cd = CallDataSerializer::new(&*b"func");
        let arg_bytes: &[u8] = &[0xff, 0xff];
        cd.push_argument_bytes(arg_bytes);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@ffff@"[..]);
    }

    #[test]
    fn test_push_some_empty_2() {
        let mut cd = CallDataSerializer::new(&*b"func");
        let arg_bytes: &[u8] = &[0xff, 0xff];
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(arg_bytes);
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@@@ffff@@"[..]);
    }

    // #[test]
    // fn test_push_i32_1() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_i32(15);
    //     assert_eq!(cd.as_slice(), &b"func@0000000f"[..]);
    // }

    // #[test]
    // fn test_push_i32_2() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_i32(256);
    //     assert_eq!(cd.as_slice(), &b"func@00000100"[..]);
    // }

    // #[test]
    // fn test_push_i32_3() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_i32(-1);
    //     assert_eq!(cd.as_slice(), &b"func@ffffffff"[..]);
    // }

    // #[test]
    // fn test_push_i32_4() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_i32(0x12345678);
    //     assert_eq!(cd.as_slice(), &b"func@12345678"[..]);
    // }

    // #[test]
    // fn test_push_i64_1() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_argument_bytes( to_bytes(&15i64).unwrap().as_slice());
    //     assert_eq!(cd.as_slice(), &b"func@000000000000000f"[..]);
    // }

    // #[test]
    // fn test_push_i64_2() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_argument_bytes(to_bytes(&256i64).unwrap().as_slice());
    //     assert_eq!(cd.as_slice(), &b"func@0000000000000100"[..]);
    // }

    // #[test]
    // fn test_push_i64_3() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_argument_bytes(to_bytes(&-1i64).unwrap().as_slice());
    //     assert_eq!(cd.as_slice(), &b"func@ffffffffffffffff"[..]);
    // }

    // #[test]
    // fn test_push_i64_4() {
    //     let mut cd = CallDataSerializer::new(&*b"func");
    //     cd.push_argument_bytes(to_bytes(&0x0123456789abcdefi64).unwrap().as_slice());
    //     assert_eq!(cd.as_slice(), &b"func@0123456789abcdef"[..]);
    // }

    
}