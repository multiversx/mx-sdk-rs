use hex_literal::hex;
use multiversx_sc_codec::{CodecFrom, EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    api::{const_handles, use_raw_handle, CallTypeApi, ManagedBufferApiImpl, ManagedTypeApi},
    proxy_imports::{ManagedRef, TxToInto},
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxScEnv, TxTo, TxToSpecified},
};

/// Address of the system smart contract that manages ESDT.
const SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");
const SYSTEM_SC_ADDRESS_BECH32: &str =
    "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

/// Indicates the system SC address, which is the same on any MultiversX blockchain.
pub struct ESDTSystemSCAddress;

impl ESDTSystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(SYSTEM_SC_ADDRESS_BYTES)
    }

    pub fn to_bech32_str(&self) -> &str {
        SYSTEM_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        SYSTEM_SC_ADDRESS_BECH32.into()
    }
}

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for ESDTSystemSCAddress
where
    Api: CallTypeApi,
{
    fn annotation(&self, _env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        ManagedBuffer::from(SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        ESDTSystemSCAddress.to_managed_address()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for ESDTSystemSCAddress where Api: CallTypeApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for ESDTSystemSCAddress where Api: CallTypeApi {}
impl<Api> TxToInto<TxScEnv<Api>> for ESDTSystemSCAddress
where
    Api: CallTypeApi,
{
    type Into = ManagedRef<'static, Api, ManagedAddress<Api>>;

    fn into_recipient(self) -> Self::Into {
        let handle: Api::ManagedBufferHandle = use_raw_handle(const_handles::ADDRESS_ESDT_SYSTEM);
        Api::managed_type_impl().mb_overwrite(handle.clone(), &SYSTEM_SC_ADDRESS_BYTES);
        unsafe { ManagedRef::wrap_handle(handle) }
    }
}

impl TopEncode for ESDTSystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        SYSTEM_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> CodecFrom<ESDTSystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for ESDTSystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(SYSTEM_SC_ADDRESS_BECH32)
    }
}
