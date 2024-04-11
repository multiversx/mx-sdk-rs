use crate::{
    api::ManagedTypeApi,
    formatter::hex_util::byte_to_hex_digits,
    types::{heap::ArgBuffer, ManagedArgBuffer, ManagedBuffer},
};
use alloc::vec::Vec;

use super::SEPARATOR;

/// Serializes to the MultiversX smart contract call format.
///
/// This format consists of the function name, followed by '@', follwed by hex-encoded argument bytes separated by '@' characters.
/// Example: "funcName@00000@aaaa@1234@@".
/// Arguments can be empty, in which case no hex digits are emitted.
/// Argument hex encodings will always have an even number of digits.
///
/// HexCallDataSerializer owns its output.
///
/// Converting from whatever type the argument to bytes is not in scope. Use the `serializer` module for that.
///
pub struct HexCallDataSerializer(Vec<u8>);

impl HexCallDataSerializer {
    pub fn new(endpoint_name: &[u8]) -> Self {
        let mut data = Vec::with_capacity(endpoint_name.len());
        data.extend_from_slice(endpoint_name);
        HexCallDataSerializer(data)
    }

    pub fn from_arg_buffer(endpoint_name: &[u8], arg_buffer: &ArgBuffer) -> Self {
        let mut hex_data = HexCallDataSerializer::new(endpoint_name);
        arg_buffer.for_each_arg(|arg_bytes| hex_data.push_argument_bytes(arg_bytes));
        hex_data
    }

    pub fn from_managed_arg_buffer<M: ManagedTypeApi>(
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Self {
        let mut hex_data = HexCallDataSerializer::new(endpoint_name.to_boxed_bytes().as_slice());
        for arg in arg_buffer.raw_arg_iter() {
            hex_data.push_argument_bytes(arg.to_boxed_bytes().as_slice());
        }
        hex_data
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    fn push_byte(&mut self, byte: u8) {
        let digits = byte_to_hex_digits(byte);
        self.0.push(digits[0]);
        self.0.push(digits[1]);
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
        let mut cd = HexCallDataSerializer::new(b"func");
        let arg_bytes: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        cd.push_argument_bytes(arg_bytes);
        assert_eq!(cd.as_slice(), &b"func@0123456789abcdef"[..]);
    }

    #[test]
    fn test_push_bytes_2() {
        let mut cd = HexCallDataSerializer::new(b"func");
        let arg_bytes: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        cd.push_argument_bytes(arg_bytes);
        cd.push_argument_bytes(arg_bytes);
        assert_eq!(
            cd.as_slice(),
            &b"func@0123456789abcdef@0123456789abcdef"[..]
        );
    }

    #[test]
    fn test_push_empty_1() {
        let mut cd = HexCallDataSerializer::new(b"func");
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@"[..]);
    }

    #[test]
    fn test_push_empty_2() {
        let mut cd = HexCallDataSerializer::new(b"func");
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@@"[..]);
    }

    #[test]
    fn test_push_empty_3() {
        let mut cd = HexCallDataSerializer::new(b"");
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"@@@"[..]);
    }

    #[test]
    fn test_push_some_empty_1() {
        let mut cd = HexCallDataSerializer::new(b"func");
        let arg_bytes: &[u8] = &[0xff, 0xff];
        cd.push_argument_bytes(arg_bytes);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@ffff@"[..]);
    }

    #[test]
    fn test_push_some_empty_2() {
        let mut cd = HexCallDataSerializer::new(b"func");
        let arg_bytes: &[u8] = &[0xff, 0xff];
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(arg_bytes);
        cd.push_argument_bytes(&[][..]);
        cd.push_argument_bytes(&[][..]);
        assert_eq!(cd.as_slice(), &b"func@@@ffff@@"[..]);
    }
}
