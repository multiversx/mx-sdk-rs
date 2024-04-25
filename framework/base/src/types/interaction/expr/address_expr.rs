use core::ptr;

use crate::types::{
    AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified, TxTo,
    TxToSpecified,
};

const ADDRESS_PREFIX: &str = "address:";

/// Encodes a dummy address, to be used for tests.
/// 
/// It is designed to be usable from contracts (especiall test contracts), with a minimal footprint.
/// For this reason, its inner structure is subject to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressExpr<'a> {
    name: &'a str,
}

impl<'a> AddressExpr<'a> {
    pub const fn new(name: &'a str) -> Self {
        AddressExpr { name }
    }

    pub const fn eval_to_array(&self) -> [u8; 32] {
        let result = [b'_'; 32];
        let expr_bytes = self.name.as_bytes();
        let mut len = expr_bytes.len();
        if len > 32 {
            len = 32;
        }
        unsafe {
            ptr::copy_nonoverlapping(expr_bytes.as_ptr(), result.as_ptr() as *mut u8, len);
        }
        result
    }

    #[cfg(feature = "alloc")]
    pub fn eval_to_expr(&self) -> alloc::string::String {
        alloc::format!("{ADDRESS_PREFIX}{}", self.name)
    }
}

impl<'a, Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for AddressExpr<'a>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBuffer::new_from_bytes(ADDRESS_PREFIX.as_bytes());
        result.append_bytes(self.name.as_bytes());
        result
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}

impl<'a, Env> TxFrom<Env> for AddressExpr<'a>
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<'a, Env> TxFromSpecified<Env> for AddressExpr<'a> where Env: TxEnv {}
impl<'a, Env> TxTo<Env> for AddressExpr<'a> where Env: TxEnv {}
impl<'a, Env> TxToSpecified<Env> for AddressExpr<'a> where Env: TxEnv {}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn assert_eq_eval(expr: &'static str, expected: &[u8; 32]) {
        assert_eq!(&AddressExpr::new(expr).eval_to_array(), expected);
    }

    #[test]
    fn test_address_value() {
        assert_eq_eval("", b"________________________________");
        assert_eq_eval("a", b"a_______________________________");
        assert_eq_eval("a\x05", b"a\x05______________________________");
        assert_eq_eval("an_address", b"an_address______________________");
        assert_eq_eval(
            "12345678901234567890123456789012",
            b"12345678901234567890123456789012",
        );
        assert_eq_eval(
            "123456789012345678901234567890123",
            b"12345678901234567890123456789012",
        );
    }
}
