use super::{FormatByteReceiver, SCBinary, SCDisplay, SCLowerHex};

impl<'a> SCDisplay<'a> for &[u8] {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}

impl<'a> SCLowerHex<'a> for &[u8] {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}

impl<'a> SCBinary<'a> for &[u8] {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}
