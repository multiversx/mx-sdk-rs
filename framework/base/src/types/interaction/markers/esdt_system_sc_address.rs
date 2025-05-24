use hex_literal::hex;
use multiversx_chain_core::types::Address;
use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxTo, TxToSpecified},
};

/// Address of the system smart contract that manages ESDT.
const ESDT_SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");
const ESDT_SYSTEM_SC_ADDRESS_BECH32: &str =
    "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const ESDT_SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

/// Indicates the system SC address, which is the same on any MultiversX blockchain.
pub struct ESDTSystemSCAddress;

impl ESDTSystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(ESDT_SYSTEM_SC_ADDRESS_BYTES)
    }

    pub fn to_address(&self) -> Address {
        ESDT_SYSTEM_SC_ADDRESS_BYTES.into()
    }

    pub fn to_bech32_str(&self) -> &str {
        ESDT_SYSTEM_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        ESDT_SYSTEM_SC_ADDRESS_BECH32.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ESDTSystemSCAddress
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(ESDT_SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        ESDTSystemSCAddress.to_managed_address()
    }
}

impl<Env> TxTo<Env> for ESDTSystemSCAddress where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ESDTSystemSCAddress where Env: TxEnv {}

impl TopEncode for ESDTSystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        ESDT_SYSTEM_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<ESDTSystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for ESDTSystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(ESDT_SYSTEM_SC_ADDRESS_BECH32)
    }
}
