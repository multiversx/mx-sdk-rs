use core::ptr;

use crate::types::{
    AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified, TxTo,
    TxToSpecified,
};

const ADDRESS_PREFIX: &str = "address:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressExpr(pub &'static str);

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for AddressExpr
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBuffer::new_from_bytes(ADDRESS_PREFIX.as_bytes());
        result.append_bytes(self.0.as_bytes());
        result
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<Env> TxFrom<Env> for AddressExpr
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<Env> TxFromSpecified<Env> for AddressExpr where Env: TxEnv {}
impl<Env> TxTo<Env> for AddressExpr where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for AddressExpr
where
    Env: TxEnv,
{
    fn with_address_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        let expr: [u8; 32] = self.eval_to_array();
        let ma = expr.into();
        f(&ma)
    }
}

impl AddressExpr {
    pub const fn eval_to_array(&self) -> [u8; 32] {
        let result = [b'_'; 32];
        let expr_bytes = self.0.as_bytes();
        let mut len = expr_bytes.len();
        if len > 32 {
            len = 32;
        }
        unsafe {
            ptr::copy_nonoverlapping(expr_bytes.as_ptr(), result.as_ptr() as *mut u8, len);
        }
        result
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn assert_eq_eval(expr: &'static str, expected: &[u8; 32]) {
        assert_eq!(&AddressExpr(expr).eval_to_array(), expected);
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
