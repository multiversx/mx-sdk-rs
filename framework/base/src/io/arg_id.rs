/// Some info to display in endpoint argument deserialization error messages,
/// to help users identify the faulty argument.
/// Generated automatically.
/// Current version uses argument names,
/// but in principle it could be changed to argument index to save some bytes from the wasm output.
#[derive(Clone, Copy)]
pub struct ArgId(&'static [u8]);

impl From<&'static [u8]> for ArgId {
    #[inline]
    fn from(static_bytes: &'static [u8]) -> Self {
        ArgId(static_bytes)
    }
}

impl From<&'static str> for ArgId {
    #[inline]
    fn from(static_str: &'static str) -> Self {
        ArgId(static_str.as_bytes())
    }
}

impl ArgId {
    pub fn as_bytes(&self) -> &'static [u8] {
        self.0
    }

    #[inline]
    pub fn empty() -> Self {
        ArgId::from(&[][..])
    }
}
