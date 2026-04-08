use super::{
    FormatByteReceiver, SCBinary, SCDisplay, SCLowerHex,
    hex_util::{byte_to_binary_digits, encode_bytes_as_hex},
};

impl SCDisplay for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}

impl SCLowerHex for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(encode_bytes_as_hex(self).as_bytes());
    }
}

impl SCBinary for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        for b in self.iter() {
            f.append_bytes(&byte_to_binary_digits(*b));
        }
    }
}
