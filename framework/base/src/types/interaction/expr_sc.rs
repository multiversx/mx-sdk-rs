use core::ptr;

use crate::{
    api::CallTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

use super::{AnnotatedValue, TxEnv, TxFrom, TxFromSpecified, TxTo, TxToSpecified};

const SC_PREFIX: &str = "sc:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScExpr<'a>(pub &'a str);

impl<'a, Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ScExpr<'a>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBuffer::new_from_bytes(SC_PREFIX.as_bytes());
        result.append_bytes(self.0.as_bytes());
        result
    }

    fn into_value(self) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }

    fn with_value_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, f: F) {
        let expr: [u8; 32] = self.eval_to_array();
        let ma = expr.into();
        f(&ma);
    }
}
impl<'a, Env> TxFrom<Env> for ScExpr<'a>
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<'a, Env> TxFromSpecified<Env> for ScExpr<'a> where Env: TxEnv {}
impl<'a, Env> TxTo<Env> for ScExpr<'a> where Env: TxEnv {}
impl<'a, Env> TxToSpecified<Env> for ScExpr<'a> where Env: TxEnv {}

impl<'a> ScExpr<'a> {
    pub const fn eval_to_array(&self) -> [u8; 32] {
        let result = *b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00______________________";
        let expr_bytes = self.0.as_bytes();
        let mut len = expr_bytes.len();
        if len > 22 {
            len = 22;
        }
        unsafe {
            ptr::copy_nonoverlapping(
                expr_bytes.as_ptr(),
                result.as_ptr().offset(10) as *mut u8,
                len,
            );
        }
        result
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn assert_eq_eval(expr: &'static str, expected: &[u8; 32]) {
        assert_eq!(&ScExpr(expr).eval_to_array(), expected);
    }

    #[test]
    fn test_address_value() {
        assert_eq_eval(
            "",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00______________________",
        );
        assert_eq_eval(
            "a",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00a_____________________",
        );
        assert_eq_eval(
            "12345678901234567890120s",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012",
        );
    }

    // #[test]
    // fn test_sc_address() {
    //     let context = InterpreterContext::default();
    //     assert_eq!(
    //         b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00a_____________________".to_vec(),
    //         interpret_string("sc:a", &context)
    //     );
    //     assert_eq!(
    //         b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012".to_vec(),
    //         interpret_string("sc:12345678901234567890120s", &context)
    //     );
    //     // trims excess
    //     assert_eq!(
    //         b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x001234567890123456789012".to_vec(),
    //         interpret_string("sc:12345678901234567890120sx", &context)
    //     );
    // }
}
