use core::ptr;

use crate::{
    api::CallTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

use super::{AnnotatedValue, TxFrom, TxFromSpecified};

const ADDRESS_PREFIX: &str = "address:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressExpr(pub &'static str);

impl<Api> AnnotatedValue<Api, ManagedAddress<Api>> for AddressExpr
where
    Api: CallTypeApi,
{
    fn annotation(&self) -> ManagedBuffer<Api> {
        let mut result = ManagedBuffer::new_from_bytes(ADDRESS_PREFIX.as_bytes());
        result.append_bytes(self.0.as_bytes());
        result
    }

    fn into_value(self) -> ManagedAddress<Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Api>)>(&self, f: F) {
        let expr: [u8; 32] = self.eval_to_array();
        let ma = expr.into();
        f(&ma);
    }
}
impl<Api> TxFrom<Api> for AddressExpr
where
    Api: CallTypeApi,
{
    fn resolve_address(&self) -> ManagedAddress<Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<Api> TxFromSpecified<Api> for AddressExpr where Api: CallTypeApi {}

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
