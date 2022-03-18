use super::{FormatByteReceiver, SCDisplay};

impl SCDisplay for &[u8] {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(self);
    }
}
