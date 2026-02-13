use multiversx_chain_core::types::Address;
use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxTo, TxToSpecified},
};

/// Address of the governance system smart contract.
const GOVERNANCE_SC_ADDRESS: Address =
    Address::from_hex("000000000000000000010000000000000000000000000000000000000003ffff");
const GOVERNANCE_SC_ADDRESS_BECH32: &str =
    "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqrlllsrujgla";
const GOVERNANCE_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqrlllsrujgla";

/// Indicates the system SC address, which is the same on any MultiversX blockchain.
pub struct GovernanceSystemSCAddress;

impl GovernanceSystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(GOVERNANCE_SC_ADDRESS)
    }

    pub fn to_address(&self) -> Address {
        GOVERNANCE_SC_ADDRESS
    }

    pub fn to_bech32_str(&self) -> &str {
        GOVERNANCE_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        GOVERNANCE_SC_ADDRESS_BECH32.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for GovernanceSystemSCAddress
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(GOVERNANCE_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        GovernanceSystemSCAddress.to_managed_address()
    }
}

impl<Env> TxTo<Env> for GovernanceSystemSCAddress where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for GovernanceSystemSCAddress where Env: TxEnv {}

impl TopEncode for GovernanceSystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        GOVERNANCE_SC_ADDRESS.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<GovernanceSystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for GovernanceSystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(GOVERNANCE_SC_ADDRESS_BECH32)
    }
}
