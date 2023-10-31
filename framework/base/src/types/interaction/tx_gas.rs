use crate::api::{BlockchainApiImpl, CallTypeApi};

use super::contract_call_exec::TRANSFER_EXECUTE_DEFAULT_LEFTOVER;

pub trait TxGas {
    fn resolve_gas<Api>(&self) -> u64
    where
        Api: CallTypeApi + 'static;
}

impl TxGas for () {
    fn resolve_gas<Api>(&self) -> u64
    where
        Api: CallTypeApi + 'static,
    {
        let mut gas_left = Api::blockchain_api_impl().get_gas_left();
        if gas_left > TRANSFER_EXECUTE_DEFAULT_LEFTOVER {
            gas_left -= TRANSFER_EXECUTE_DEFAULT_LEFTOVER;
        }
        gas_left
    }
}

pub struct ExplicitGas(pub u64);

impl TxGas for ExplicitGas {
    fn resolve_gas<Api>(&self) -> u64
    where
        Api: CallTypeApi + 'static,
    {
        self.0
    }
}
