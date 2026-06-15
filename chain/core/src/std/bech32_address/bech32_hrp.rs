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

impl TryFrom<String> for Bech32Hrp {
    type Error = bech32::primitives::hrp::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Hrp::parse(&s).map(Bech32Hrp)
    }
}

impl TryFrom<&str> for Bech32Hrp {
    type Error = bech32::primitives::hrp::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Hrp::parse(s).map(Bech32Hrp)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_str_valid() {
        let hrp = Bech32Hrp::try_from("erd").unwrap();
        assert_eq!(hrp.as_str(), "erd");

        let hrp = Bech32Hrp::try_from("test").unwrap();
        assert_eq!(hrp.as_str(), "test");
    }

    #[test]
    fn test_try_from_string_valid() {
        let hrp = Bech32Hrp::try_from(String::from("erd")).unwrap();
        assert_eq!(hrp.as_str(), "erd");
    }

    #[test]
    fn test_try_from_str_invalid() {
        // HRP cannot be empty
        assert!(Bech32Hrp::try_from("").is_err());
        // HRP cannot contain non-ASCII characters
        assert!(Bech32Hrp::try_from("\u{e9}rd").is_err());
    }

    #[test]
    fn test_try_from_string_invalid() {
        assert!(Bech32Hrp::try_from(String::from("")).is_err());
        assert!(Bech32Hrp::try_from(String::from("\u{e9}rd")).is_err());
    }

    #[test]
    fn test_parse_unchecked() {
        let hrp = Bech32Hrp::parse_unchecked("erd");
        assert_eq!(hrp.as_str(), "erd");
    }

    #[test]
    fn test_default_is_erd() {
        assert_eq!(Bech32Hrp::default().as_str(), "erd");
    }

    #[test]
    fn test_partial_eq_str() {
        let hrp = Bech32Hrp::try_from("erd").unwrap();
        assert!(hrp == *"erd");
        assert!(hrp == "erd");
        assert!(hrp != *"test");
        assert!(hrp != "test");
    }

    #[test]
    fn test_display() {
        let hrp = Bech32Hrp::try_from("erd").unwrap();
        assert_eq!(hrp.to_string(), "erd");
    }

    #[test]
    fn test_serde_roundtrip() {
        let hrp = Bech32Hrp::try_from("erd").unwrap();
        let json = serde_json::to_string(&hrp).unwrap();
        assert_eq!(json, "\"erd\"");
        let decoded: Bech32Hrp = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, hrp);
    }

    #[test]
    fn test_serde_deserialize_invalid() {
        // Non-ASCII characters are not valid in an HRP
        assert!(serde_json::from_str::<Bech32Hrp>("\"\u{e9}rd\"").is_err());
    }
}
