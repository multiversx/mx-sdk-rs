use super::{FormatByteReceiver, SCBinary, SCDisplay, SCLowerHex};

impl SCDisplay for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}

impl SCLowerHex for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}

impl SCBinary for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}
