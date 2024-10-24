use multiversx_chain_core::types::CodeMetadata;

use super::{hex_util, FormatByteReceiver, SCBinary, SCDisplay, SCLowerHex};

impl SCDisplay for CodeMetadata {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        self.for_each_string_token(|token| f.append_bytes(token.as_bytes()))
    }
}

impl SCLowerHex for CodeMetadata {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let num = self.bits().to_be_bytes();
        f.append_bytes(&hex_util::byte_to_hex_digits(num[0])[..]);
        f.append_bytes(&hex_util::byte_to_hex_digits(num[1])[..]);
    }
}

impl SCBinary for CodeMetadata {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let num = self.bits().to_be_bytes();
        f.append_bytes(&hex_util::byte_to_binary_digits(num[0])[..]);
        f.append_bytes(&hex_util::byte_to_binary_digits(num[1])[..]);
    }
}
