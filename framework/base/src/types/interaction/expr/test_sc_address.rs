use core::ptr;

use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{
        heap::Address, AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom,
        TxFromSpecified, TxTo, TxToSpecified,
    },
};

use super::TestAddress;

const SC_PREFIX: &str = "sc:";
const VM_TYPE_LEN: usize = 2;
const DEFAULT_VM_TYPE: &[u8] = &[5, 0];

/// Encodes a dummy SC address, to be used for tests.
///
/// It is designed to be usable from contracts (especiall test contracts), with a minimal footprint.
/// For this reason, its inner structure is subject to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestSCAddress<'a> {
    name: &'a str,
}

impl<'a> TestSCAddress<'a> {
    pub const fn new(name: &'a str) -> Self {
        TestSCAddress { name }
    }
}

impl<'a, Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for TestSCAddress<'a>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBuffer::new_from_bytes(SC_PREFIX.as_bytes());
        result.append_bytes(self.name.as_bytes());
        result
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}

impl<'a> TestSCAddress<'a> {
    pub fn to_address(&self) -> Address {
        self.eval_to_array().into()
    }

    pub fn to_managed_address<Api: ManagedTypeApi>(&self) -> ManagedAddress<Api> {
        self.eval_to_array().into()
    }
}

impl<'a, 'b> PartialEq<TestAddress<'b>> for TestSCAddress<'a> {
    fn eq(&self, other: &TestAddress) -> bool {
        self.to_address() == other.to_address()
    }
}

impl<'a> PartialEq<Address> for TestSCAddress<'a> {
    fn eq(&self, other: &Address) -> bool {
        &self.to_address() == other
    }
}

impl<'a> PartialEq<TestSCAddress<'a>> for Address {
    fn eq(&self, other: &TestSCAddress<'a>) -> bool {
        self == &other.to_address()
    }
}

impl<'a, Api: ManagedTypeApi> PartialEq<ManagedAddress<Api>> for TestSCAddress<'a> {
    fn eq(&self, other: &ManagedAddress<Api>) -> bool {
        self.to_address() == other.to_address()
    }
}

impl<'a, Api: ManagedTypeApi> PartialEq<TestSCAddress<'a>> for ManagedAddress<Api> {
    fn eq(&self, other: &TestSCAddress<'a>) -> bool {
        self.to_address() == other.to_address()
    }
}

impl<'a, Env> TxFrom<Env> for TestSCAddress<'a>
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<'a, Env> TxFromSpecified<Env> for TestSCAddress<'a> where Env: TxEnv {}
impl<'a, Env> TxTo<Env> for TestSCAddress<'a> where Env: TxEnv {}
impl<'a, Env> TxToSpecified<Env> for TestSCAddress<'a> where Env: TxEnv {}

impl<'a> TestSCAddress<'a> {
    pub fn eval_to_array(&self) -> [u8; 32] {
        let result = *b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00______________________";
        let expr_bytes = self.name.as_bytes();
        let mut len = expr_bytes.len();
        if len > 22 {
            len = 22;
        }
        unsafe {
            ptr::copy_nonoverlapping(
                DEFAULT_VM_TYPE.as_ptr(),
                result.as_ptr().offset(8) as *mut u8,
                VM_TYPE_LEN,
            );
            ptr::copy_nonoverlapping(
                expr_bytes.as_ptr(),
                result.as_ptr().offset(10) as *mut u8,
                len,
            );
        }
        result
    }

    #[cfg(feature = "alloc")]
    pub fn eval_to_expr(&self) -> alloc::string::String {
        alloc::format!("{SC_PREFIX}{}", self.name)
    }
}

impl<'a> TopEncode for TestSCAddress<'a> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.eval_to_array().top_encode_or_handle_err(output, h)
    }
}

impl<'a, Api> TypeAbiFrom<TestSCAddress<'a>> for ManagedAddress<Api> where Api: ManagedTypeApi {}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn assert_eq_eval(expr: &'static str, expected: &[u8; 32]) {
        assert_eq!(&TestSCAddress::new(expr).eval_to_array(), expected);
    }

    #[test]
    fn test_address_value() {
        assert_eq_eval(
            "",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x05\x00______________________",
        );
        assert_eq_eval(
            "a",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x05\x00a_____________________",
        );
        assert_eq_eval(
            "12345678901234567890120s",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x05\x001234567890123456789012",
        );
    }
}
