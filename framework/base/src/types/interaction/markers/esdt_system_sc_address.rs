use hex_literal::hex;

use crate::{
    api::{
        const_handles, use_raw_handle, BlockchainApi, BlockchainApiImpl, CallTypeApi,
        ManagedTypeApi,
    },
    contract_base::BlockchainWrapper,
    types::{
        AnnotatedValue, ManagedAddress, ManagedBuffer, ManagedType, TxScEnv, TxTo, TxToSpecified,
    },
};

/// Address of the system smart contract that manages ESDT.
const SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");
const SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

/// Indicates the system SC address, which is the same on any MultiversX blockchain.
pub struct ESDTSystemSCAddress;

impl ESDTSystemSCAddress {
    pub fn managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(SYSTEM_SC_ADDRESS_BYTES)
    }
}

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for ESDTSystemSCAddress
where
    Api: CallTypeApi,
{
    fn annotation(&self, env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        ManagedBuffer::from(SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        ESDTSystemSCAddress.managed_address()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for ESDTSystemSCAddress where Api: CallTypeApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for ESDTSystemSCAddress where Api: CallTypeApi {}
