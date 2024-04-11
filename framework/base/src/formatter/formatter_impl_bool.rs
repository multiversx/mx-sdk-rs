use super::{FormatByteReceiver, SCDisplay};

const TRUE_BYTES: &[u8] = b"true";
const FALSE_BYTES: &[u8] = b"false";

impl<'a> SCDisplay<'a> for bool {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        if *self {
            f.append_bytes(TRUE_BYTES);
        } else {
            f.append_bytes(FALSE_BYTES);
        }
    }
}
