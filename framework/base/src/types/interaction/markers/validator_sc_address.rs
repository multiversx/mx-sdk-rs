use hex_literal::hex;
use multiversx_chain_core::types::Address;
use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxTo, TxToSpecified},
};

/// Address of the validator smart contract that manages ESDT.
const VALIDATOR_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000001ffff");
const VALIDATOR_SC_ADDRESS_BECH32: &str =
    "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqplllst77y4l";
const VALIDATOR_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqplllst77y4l";

/// Indicates the system SC address, which is the same on any MultiversX blockchain.
pub struct ValidatorSCAddress;

impl ValidatorSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(VALIDATOR_SC_ADDRESS_BYTES)
    }

    pub fn to_address(&self) -> Address {
        VALIDATOR_SC_ADDRESS_BYTES.into()
    }

    pub fn to_bech32_str(&self) -> &str {
        VALIDATOR_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        VALIDATOR_SC_ADDRESS_BECH32.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ValidatorSCAddress
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from(VALIDATOR_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        ValidatorSCAddress.to_managed_address()
    }
}

impl<Env> TxTo<Env> for ValidatorSCAddress where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ValidatorSCAddress where Env: TxEnv {}

impl TopEncode for ValidatorSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        VALIDATOR_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<ValidatorSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for ValidatorSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(VALIDATOR_SC_ADDRESS_BECH32)
    }
}
