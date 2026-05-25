use std::borrow::Cow;

/// Newtype wrapping the human-readable part (HRP) of a bech32 address.
///
/// Uses [`Cow<'static, str>`] so that constants can be created with borrowed string literals:
///
/// ```rust
/// # use multiversx_chain_core::std::Bech32Hrp;
/// use std::borrow::Cow;
/// const ERD_HRP: Bech32Hrp = Bech32Hrp(Cow::Borrowed("erd"));
/// ```
///
/// Dynamic HRPs built at runtime use the [`Cow::Owned`] variant via [`Bech32Hrp::from_string`].
#[derive(Clone, PartialEq, Eq)]
pub struct Bech32Hrp(pub Cow<'static, str>);

impl Bech32Hrp {
    /// Creates a `Bech32Hrp` from a `&'static str`.
    ///
    /// This is a `const fn`, making it usable in constant and static contexts:
    /// ```rust
    /// # use multiversx_chain_core::std::Bech32Hrp;
    /// const ERD: Bech32Hrp = Bech32Hrp::from_static("erd");
    /// ```
    pub const fn from_static(hrp: &'static str) -> Self {
        Bech32Hrp(Cow::Borrowed(hrp))
    }

    /// Creates a `Bech32Hrp` from an owned [`String`].
    pub fn from_string(hrp: String) -> Self {
        Bech32Hrp(Cow::Owned(hrp))
    }

    /// Returns the HRP as a borrowed `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Bech32Hrp {
    fn from(s: String) -> Self {
        Bech32Hrp::from_string(s)
    }
}

impl From<&'static str> for Bech32Hrp {
    fn from(s: &'static str) -> Self {
        Bech32Hrp::from_static(s)
    }
}

impl PartialEq<str> for Bech32Hrp {
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref() == other
    }
}

impl PartialEq<&str> for Bech32Hrp {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_ref() == *other
    }
}

impl core::fmt::Display for Bech32Hrp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::fmt::Debug for Bech32Hrp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Bech32Hrp").field(&self.0).finish()
    }
}
