use multiversx_chain_core::types::Address;
use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxTo, TxToSpecified},
};

/// Address of the System Smart Contract.
const SYSTEM_SC_ADDRESS: Address =
    Address::from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

/// Bech32-encoded address of the System Smart Contract.
const SYSTEM_SC_ADDRESS_BECH32: &str =
    "erd1lllllllllllllllllllllllllllllllllllllllllllllllllllsckry7t";

/// Annotation for Bech32 format.
const SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1lllllllllllllllllllllllllllllllllllllllllllllllllllsckry7t";

/// Indicates the System SC address, which is the same on any MultiversX blockchain.
pub struct SystemSCAddress;

impl SystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(SYSTEM_SC_ADDRESS)
    }

    pub fn to_address(&self) -> Address {
        SYSTEM_SC_ADDRESS
    }

    pub fn to_bech32_str(&self) -> &str {
        SYSTEM_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        SYSTEM_SC_ADDRESS_BECH32.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for SystemSCAddress
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        SystemSCAddress.to_managed_address()
    }
}

impl<Env> TxTo<Env> for SystemSCAddress where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for SystemSCAddress where Env: TxEnv {}

impl TopEncode for SystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        SYSTEM_SC_ADDRESS.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<SystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for SystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(SYSTEM_SC_ADDRESS_BECH32)
    }
}
