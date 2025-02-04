use core::ptr;

use multiversx_sc_codec::{
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{
        heap::Address, AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom,
        TxFromSpecified, TxTo, TxToSpecified,
    },
};

use super::TestSCAddress;

const ADDRESS_PREFIX: &str = "address:";

/// Encodes a dummy address, to be used for tests.
///
/// It is designed to be usable from contracts (especiall test contracts), with a minimal footprint.
/// For this reason, its inner structure is subject to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestAddress<'a> {
    name: &'a str,
}

impl<'a> TestAddress<'a> {
    pub const fn new(name: &'a str) -> Self {
        TestAddress { name }
    }

    pub fn eval_to_array(&self) -> [u8; 32] {
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

    pub fn to_address(&self) -> Address {
        self.eval_to_array().into()
    }

    pub fn to_managed_address<Api: ManagedTypeApi>(&self) -> ManagedAddress<Api> {
        self.eval_to_array().into()
    }

    #[cfg(feature = "alloc")]
    pub fn eval_to_expr(&self) -> alloc::string::String {
        alloc::format!("{ADDRESS_PREFIX}{}", self.name)
    }
}

impl PartialEq<TestSCAddress<'_>> for TestAddress<'_> {
    fn eq(&self, other: &TestSCAddress) -> bool {
        self.to_address() == other.to_address()
    }
}

impl PartialEq<Address> for TestAddress<'_> {
    fn eq(&self, other: &Address) -> bool {
        &self.to_address() == other
    }
}

impl<'a> PartialEq<TestAddress<'a>> for Address {
    fn eq(&self, other: &TestAddress<'a>) -> bool {
        self == &other.to_address()
    }
}

impl<Api: ManagedTypeApi> PartialEq<ManagedAddress<Api>> for TestAddress<'_> {
    fn eq(&self, other: &ManagedAddress<Api>) -> bool {
        self.to_address() == other.to_address()
    }
}

impl<'a, Api: ManagedTypeApi> PartialEq<TestAddress<'a>> for ManagedAddress<Api> {
    fn eq(&self, other: &TestAddress<'a>) -> bool {
        self.to_address() == other.to_address()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for TestAddress<'_>
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

impl<Env> TxFrom<Env> for TestAddress<'_>
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        let expr: [u8; 32] = self.eval_to_array();
        expr.into()
    }
}
impl<Env> TxFromSpecified<Env> for TestAddress<'_> where Env: TxEnv {}
impl<Env> TxTo<Env> for TestAddress<'_> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for TestAddress<'_> where Env: TxEnv {}

impl TopEncode for TestAddress<'_> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.eval_to_array().top_encode_or_handle_err(output, h)
    }
}

impl NestedEncode for TestAddress<'_> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.eval_to_array().dep_encode_or_handle_err(dest, h)
    }
}

impl<Api> TypeAbiFrom<TestAddress<'_>> for ManagedAddress<Api> where Api: ManagedTypeApi {}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn assert_eq_eval(expr: &'static str, expected: &[u8; 32]) {
        assert_eq!(&TestAddress::new(expr).eval_to_array(), expected);
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
