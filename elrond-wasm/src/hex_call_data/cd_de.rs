use super::SEPARATOR;
use crate::{err_msg, types::StaticSCError};
use alloc::vec::Vec;

fn hex_digit_to_half_byte(digit: u8) -> Option<u8> {
    if (b'0'..=b'9').contains(&digit) {
        return Some(digit - b'0');
    }
    if (b'a'..=b'f').contains(&digit) {
        return Some(digit - b'a' + 0xau8);
    }
    None
}

fn hex_to_byte(digit1: u8, digit2: u8) -> Option<u8> {
    let mut result: u8;
    match hex_digit_to_half_byte(digit1) {
        None => {
            return None;
        },
        Some(num) => {
            result = num << 4;
        },
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

/// Deserializes from Elrond's smart contract call format.
///
/// This format consists of the function name, followed by '@', follwed by hex-encoded argument bytes separated by '@' characters.
/// Example: "funcName@00000@aaaa@1234@@".
/// Arguments can be empty.
/// Argument hex encodings must always have an even number of digits.
///
/// HexCallDataDeserializer borrows its input and will allocate new Vecs for each output.
///
/// Converting from bytes to specific argument types is not in scope. Use the `serializer` module for that.
///
pub struct HexCallDataDeserializer<'a> {
    source: &'a [u8],
    index: usize,
    func_name_output: &'a [u8],
}

impl<'a> HexCallDataDeserializer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        let mut de = HexCallDataDeserializer {
            source,
            index: 0,
            func_name_output: &[],
        };

        // extract func name and advance index, before any argument can be retrieved
        if let Some(func_name) = de.next_argument_hex() {
            de.func_name_output = func_name
        }

        de
    }

    /// Gets the first component of the call data, which is the function name.
    /// Unlike the arguments, this can be called at any time.
    #[inline]
    pub fn get_func_name(&self) -> &'a [u8] {
        self.func_name_output
    }

    fn next_argument_hex(&mut self) -> Option<&'a [u8]> {
        let initial_index = self.index;
        loop {
            if !self.has_next() {
                return None;
            }

            if self.index == self.source.len() {
                let slice = &self.source[initial_index..self.index];
                self.index += 1; // make index = len + 1 to signal that we are done, and return None from the next call on
                return Some(slice);
            }

            let c = self.source[self.index];
            if c == SEPARATOR {
                let slice = &self.source[initial_index..self.index];
                self.index += 1;
                return Some(slice);
            }

            self.index += 1;
        }
    }

    #[inline]
    pub fn has_next(&self) -> bool {
        self.index <= self.source.len()
    }

    /// Gets the next argument, deserializes from hex and returns the resulting bytes.
    pub fn next_argument(&mut self) -> Result<Option<Vec<u8>>, StaticSCError> {
        match self.next_argument_hex() {
            None => Ok(None),
            Some(arg_hex) => {
                if arg_hex.len() % 2 != 0 {
                    return Err(StaticSCError::from(err_msg::DESERIALIZATION_ODD_DIGITS));
                }
                let res_len = arg_hex.len() / 2;
                let mut res_vec = Vec::with_capacity(res_len);
                for i in 0..res_len {
                    match hex_to_byte(arg_hex[2 * i], arg_hex[2 * i + 1]) {
                        None => {
                            return Err(StaticSCError::from(err_msg::DESERIALIZATION_INVALID_BYTE));
                        },
                        Some(byte) => {
                            res_vec.push(byte);
                        },
                    }
                }
                Ok(Some(res_vec))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_raw_bytes_1() {
        let input: &[u8] = b"func@1111@2222";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.get_func_name(), &b"func"[..]);
        assert_eq!(de.next_argument_hex(), Some(&b"1111"[..]));
        assert_eq!(de.next_argument(), Ok(Some([0x22, 0x22].to_vec())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_raw_bytes_empty() {
        let mut de = HexCallDataDeserializer::new(&[]);
        assert_eq!(de.get_func_name(), &[][..]);
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_raw_bytes_only_func() {
        let input: &[u8] = b"func";
        let mut de = HexCallDataDeserializer::new(input);

        assert_eq!(de.get_func_name(), &b"func"[..]);
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_raw_bytes_some_empty() {
        let input: &[u8] = b"func@@2222";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some([0x22, 0x22].to_vec())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));

        assert_eq!(de.get_func_name(), &b"func"[..]);
    }

    #[test]
    fn test_next_raw_bytes_ends_empty() {
        let input: &[u8] = b"func@";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.get_func_name(), &b"func"[..]);
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_raw_bytes_many_empty() {
        let input: &[u8] = b"func@@2222@@";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.get_func_name(), &b"func"[..]);
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some([0x22, 0x22].to_vec())));
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_raw_bytes_all_empty() {
        let input: &[u8] = b"@@@";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.get_func_name(), &[][..]);
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_raw_bytes_all_empty_but_last() {
        let input: &[u8] = b"@@@1234";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.get_func_name(), &[][..]);
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some(Vec::new())));
        assert_eq!(de.next_argument(), Ok(Some([0x12, 0x34].to_vec())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_argument_large() {
        let input: &[u8] = b"func@0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let mut de = HexCallDataDeserializer::new(input);
        let expected: [u8; 32] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
            0x89, 0xab, 0xcd, 0xef,
        ];
        assert_eq!(de.get_func_name(), &b"func"[..]);
        assert!(de.next_argument() == Ok(Some(expected.to_vec())));
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }

    #[test]
    fn test_next_vec_odd() {
        let input: &[u8] = b"func@123";
        let mut de = HexCallDataDeserializer::new(input);
        assert_eq!(de.get_func_name(), &b"func"[..]);
        assert_eq!(
            de.next_argument(),
            Err(StaticSCError::from(err_msg::DESERIALIZATION_ODD_DIGITS))
        );
        assert_eq!(de.next_argument(), Ok(None));
        assert_eq!(de.next_argument(), Ok(None));
    }
}
