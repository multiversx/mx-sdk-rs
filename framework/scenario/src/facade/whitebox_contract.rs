use multiversx_sc::contract_base::{CallableContract, ContractBase};

use crate::{scenario_model::AddressKey, DebugApi};

/// Wraps a contract that is supposed to be used in whitebox tests.
///
/// For this reason it references the concrete SC type explicitly.
pub struct WhiteboxContract<ContractObj>
where
    ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
{
    pub address_expr: AddressKey,
    pub contract_obj_builder: fn() -> ContractObj,
}

impl<ContractObj> WhiteboxContract<ContractObj>
where
    ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
{
    pub fn new<A: Into<AddressKey>>(
        address_expr: A,
        contract_obj_builder: fn() -> ContractObj,
    ) -> Self {
        Self {
            address_expr: address_expr.into(),
            contract_obj_builder,
        }
    }
}
