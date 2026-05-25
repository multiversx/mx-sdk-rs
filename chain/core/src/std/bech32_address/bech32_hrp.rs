use bech32::Hrp;
use serde::{Deserialize, Serialize};

/// The default human-readable part (HRP) for MultiversX mainnet addresses: `"erd"`.
///
/// Used as the [`Default`] value for [`Bech32Hrp`] and as the default HRP when
/// encoding addresses without an explicit HRP.
pub const ERD_HRP: Bech32Hrp = Bech32Hrp::parse_unchecked("erd");

/// Newtype wrapping the human-readable part (HRP) of a bech32 address.
///
/// Wraps [`bech32::Hrp`], which validates and stores the HRP in a fixed-size buffer.
/// Because [`Hrp`] is [`Copy`], so is `Bech32Hrp`.
///
/// Constants can be created with [`Bech32Hrp::parse_unchecked`], which is `const`:
///
/// ```rust
/// # use multiversx_chain_core::std::Bech32Hrp;
/// const ERD_HRP: Bech32Hrp = Bech32Hrp::parse_unchecked("erd");
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bech32Hrp(Hrp);

impl Bech32Hrp {
    /// Creates a `Bech32Hrp` from a string slice without validation.
    ///
    /// This is a `const fn`, making it usable in constant and static contexts:
    /// ```rust
    /// # use multiversx_chain_core::std::Bech32Hrp;
    /// const ERD: Bech32Hrp = Bech32Hrp::parse_unchecked("erd");
    /// ```
    ///
    /// For runtime strings with validation use [`Bech32Hrp::from_string`].
    pub const fn parse_unchecked(hrp: &str) -> Self {
        Bech32Hrp(Hrp::parse_unchecked(hrp))
    }

    /// Parses and creates a `Bech32Hrp` from an owned [`String`].
    ///
    /// # Panics
    ///
    /// Panics if `hrp` is not a valid bech32 human-readable part.
    pub fn from_string(hrp: String) -> Self {
        Bech32Hrp(Hrp::parse(&hrp).unwrap_or_else(|err| panic!("invalid HRP '{hrp}': {err}")))
    }

    /// Returns the HRP as a borrowed `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_inner(&self) -> Hrp {
        self.0
    }
}

impl Default for Bech32Hrp {
    fn default() -> Self {
        ERD_HRP
    }
}

impl From<Hrp> for Bech32Hrp {
    fn from(hrp: Hrp) -> Self {
        Bech32Hrp(hrp)
    }
}

impl From<String> for Bech32Hrp {
    fn from(s: String) -> Self {
        Bech32Hrp::from_string(s)
    }
}

impl From<&str> for Bech32Hrp {
    fn from(s: &str) -> Self {
        Bech32Hrp::parse_unchecked(s)
    }
}

impl PartialEq<str> for Bech32Hrp {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
    }
}

impl PartialEq<&str> for Bech32Hrp {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl core::fmt::Display for Bech32Hrp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::fmt::Debug for Bech32Hrp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Bech32Hrp").field(&self.0.as_str()).finish()
    }
}

impl Serialize for Bech32Hrp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bech32Hrp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Hrp::parse(&s)
            .map(Bech32Hrp)
            .map_err(|e| serde::de::Error::custom(format!("invalid HRP: {e}")))
    }
}
