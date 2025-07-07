use hex_literal::hex;
use multiversx_chain_core::types::Address;
use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxTo, TxToSpecified},
};

/// Address of the delegation manager system smart contract.
const DELEGATION_MANAGER_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000004ffff");
const DELEGATION_MANAGER_SC_ADDRESS_BECH32: &str =
    "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqylllslmq6y6";
const DELEGATION_MANAGER_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqylllslmq6y6";

/// Indicates the delegation manager SC address, which is the same on any MultiversX blockchain.
pub struct DelegationManagerSCAddress;

impl DelegationManagerSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(DELEGATION_MANAGER_SC_ADDRESS_BYTES)
    }

    pub fn to_address(&self) -> Address {
        DELEGATION_MANAGER_SC_ADDRESS_BYTES.into()
    }

    pub fn to_bech32_str(&self) -> &str {
        DELEGATION_MANAGER_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        DELEGATION_MANAGER_SC_ADDRESS_BECH32.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for DelegationManagerSCAddress
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(DELEGATION_MANAGER_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        DelegationManagerSCAddress.to_managed_address()
    }
}

impl<Env> TxTo<Env> for DelegationManagerSCAddress where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for DelegationManagerSCAddress where Env: TxEnv {}

impl TopEncode for DelegationManagerSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        DELEGATION_MANAGER_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<DelegationManagerSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for DelegationManagerSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(DELEGATION_MANAGER_SC_ADDRESS_BECH32)
    }
}
